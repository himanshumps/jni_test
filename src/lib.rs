

// This is the interface to the JVM that we'll call the majority of our
// methods on.
// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use serde_json::Value;
use jni::{JNIEnv};
use jni::objects::{JClass, JString};

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_com_test_large_response_JsonOperationsViaJNI_removekeys<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, input: JString<'local>) -> JString<'local> {
	let str: String = env.get_string(&input).expect("Couldn't get java string!").into();
	//println!("Got String: {}", &str);
	// Convert String to serde hashmap
	let mut json: Value = serde_json::from_str(&str).unwrap();
    let _ = json.as_object_mut().unwrap().remove("index");
    let _ = json.as_object_mut().unwrap().remove("eyeColor");
    let output = env.new_string(serde_json::to_string(&json).unwrap()).expect("Issue....");
    output
}