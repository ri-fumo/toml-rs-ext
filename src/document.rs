use godot::prelude::*;
use toml::{ value::{self, Datetime, Offset}, Table, Value };


#[derive(GodotClass)]
#[class(init, base=RefCounted)]
struct TomlDocument {
	root: Option<Gd<TomlTable>>,
	base: Base<RefCounted>
}

#[godot_api]
impl TomlDocument {
	#[func]
	fn parse(str: GString) -> Option<Gd<TomlDocument>> {
		let table = str.to_string().parse::<Table>();
		match table {
			Err(err) => {
				godot_error!("{err:?}");
				None
			},
			Ok(table) => {
				let mut this = Self::new_gd();
				let mut this_mut = this.bind_mut();
				this_mut.root = Some(TomlTable::from_table(&table));
				drop(this_mut);
				Some(this)
			}
		}
	}
	#[func]
	fn get_root(&self) -> Option<Gd<TomlTable>> {
		self.root.clone()
	}
}

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
struct TomlTable {
	table: Option<Table>,
	base: Base<RefCounted>
}

#[godot_api]
impl TomlTable {
	#[func]
	fn get(&self, key: GString) -> Variant {
		if let Some(table) = self.table.as_ref() {
			if let Some(value) = table.get(&key.to_string()) {
				return value2variant(value)
			}
		}
		Variant::nil()
	}
	#[func]
	fn to_dict(&self) -> Dictionary {
		if let Some(table) = self.table.as_ref() {
			return table2dict(table)
		}
		Dictionary::new()
	}

	fn from_table(table: &Table) -> Gd<TomlTable> {
		let mut node = TomlTable::new_gd();
		let mut node_mut = node.bind_mut();
		node_mut.table = Some(table.clone());
		drop(node_mut);
		node
	}
}

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
struct TomlArray {
	array: Option<value::Array>,
	base: Base<RefCounted>
}

#[godot_api]
impl TomlArray {
	#[func]
	fn get(&self, index: i64) -> Variant {
		if let Some(array) = self.array.as_ref() {
			if let Some(value) = array.get(index as usize) {
				return value2variant(value)
			}
		}
		Variant::nil()
	}
	#[func]
	fn to_array(&self) -> VariantArray {
		let mut arr = VariantArray::new();
		if let Some(array) = self.array.as_ref() {
			for value in array {
				arr.push(&value2variant(value));
			}
		}
		arr
	}

	fn from_array(array: &value::Array) -> Gd<TomlArray> {
		let mut node = TomlArray::new_gd();
		let mut node_mut = node.bind_mut();
		node_mut.array = Some(array.clone());
		drop(node_mut);
		node
	}
}

fn table2dict(table: &Table) -> Dictionary {
	let mut dict = Dictionary::new();
	for (key, value) in table.iter() {
		let key = key.to_godot();
		match value {
			Value::String(s) => dict.set(key, s.to_godot()),
			Value::Integer(i) => dict.set(key, i.to_godot()),
			Value::Float(f) => dict.set(key, f.to_godot()),
			Value::Boolean(b) => dict.set(key, b.to_godot()),
			Value::Datetime(d) => dict.set(key, datetime2dict(d)),
			Value::Array(array) => dict.set(key, array2varray(array)),
			Value::Table(table) => dict.set(key, table2dict(table)),
		};
	}
	dict
}

fn array2varray(array: &Vec<Value>) -> VariantArray {
	let mut arr = VariantArray::new();
	for value in array.iter() {
		match value {
			Value::String(s) => arr.push(&s.to_variant()),
			Value::Integer(i) => arr.push(&i.to_variant()),
			Value::Float(f) => arr.push(&f.to_variant()),
			Value::Boolean(b) => arr.push(&b.to_variant()),
			Value::Datetime(d) => arr.push(&datetime2dict(d).to_variant()),
			Value::Array(array) => arr.push(&array2varray(array).to_variant()),
			Value::Table(table) => arr.push(&table2dict(table).to_variant()),
		};
	}
	arr
}

fn value2variant(value: &Value) -> Variant {
	match value {
		Value::String(s) => s.to_variant(),
		Value::Integer(i) => i.to_variant(),
		Value::Float(f) => f.to_variant(),
		Value::Boolean(b) => b.to_variant(),
		Value::Datetime(d) => datetime2dict(d).to_variant(),
		Value::Array(array) => TomlArray::from_array(array).to_variant(),
		Value::Table(table) => TomlTable::from_table(table).to_variant(),
	}
}

fn datetime2dict(datetime: &Datetime) -> Dictionary {
	let mut dict = Dictionary::new();
	if let Some(date) = datetime.date {
		dict.set("year", date.year);
		dict.set("month", date.month);
		dict.set("day", date.day);
		dict.set("weekday", 
					date2weekday(date.year.into(), date.month.into(), date.day.into()));
	}
	if let Some(time) = datetime.time {
		dict.set("hour", time.hour);
		dict.set("minute", time.minute);
		dict.set("second", time.second);
		dict.set("nanosecond", time.nanosecond);
	}
	if let Some(offset) = datetime.offset {
		if let Offset::Custom { minutes } = offset {
			dict.set("bias", minutes);
		} else {
			dict.set("bias", 0);
		}
	}
	dict
}

fn date2weekday(mut y: i32, mut m: i32, d:i32) -> i32 {
	if m < 3 {
			m += 12;
			y -= 1;
	}

	let k = y % 100;
	let j = y / 100;

	let f = d + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j;
	(f + 5)%7
}
