use gdnative::*;

extern crate gdnative as godot;

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl HelloWorld {
    
    /// The "constructor" of the class.
    fn _init(_owner: Node) -> Self {
        HelloWorld
    }
    
    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are"attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    fn _ready(&self, _owner: Node) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("hello, world...");
    }
}



#[derive(godot::NativeClass)]
#[inherit(godot::MeshInstance)]
#[user_data(godot::user_data::MutexData<RustTest>)]
#[register_with(my_register_function)]
struct RustTest {
    start: godot::Vector3,
    time: f32,
    #[property(default = 0.05)]
    rotate_speed: f64,
}

fn my_register_function(builder: &godot::init::ClassBuilder<RustTest>) {
    builder.add_property(godot::init::Property {
        name: "test/test_enum",
        default: godot::GodotString::from_str("Hello"),
        hint: godot::init::PropertyHint::Enum {
            values: &["Hello", "World", "Testing"],
        },
        getter: |_: &RustTest| godot::GodotString::from_str("Hello"),
        setter: (),
        usage: godot::init::PropertyUsage::DEFAULT,
    });
    builder.add_property(godot::init::Property {
        name: "test/test_flags",
        default: 0,
        hint: godot::init::PropertyHint::Flags {
            values: &["A", "B", "C", "D"],
        },
        getter: |_: &RustTest| 0,
        setter: (),
        usage: godot::init::PropertyUsage::DEFAULT,
    });
}

#[godot::methods]
impl RustTest {
    fn _init(_owner: godot::MeshInstance) -> Self {
        RustTest {
            start: godot::Vector3::new(0.0, 0.0, 0.0),
            time: 0.0,
            rotate_speed: 0.05,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: godot::MeshInstance) {
        owner.set_physics_process(true);
        self.start = owner.get_translation();
        godot_warn!("Start: {:?}", self.start);
        godot_warn!(
            "Parent name: {:?}",
            owner.get_parent().expect("Missing parent").get_name()
        );
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: godot::MeshInstance, delta: f64) {
        use godot::{Color, SpatialMaterial, Vector3};

        self.time += delta as f32;
        owner.rotate_y(self.rotate_speed * delta);

        let offset = Vector3::new(0.0, 1.0, 0.0) * self.time.cos() * 0.5;
        owner.set_translation(self.start + offset);

        if let Some(mat) = owner.get_surface_material(0) {
            let mut mat = mat.cast::<SpatialMaterial>().expect("Incorrect material");
            mat.set_albedo(Color::rgba(self.time.cos().abs(), 0.0, 0.0, 1.0));
        }
    }
}


// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<HelloWorld>();
    handle.add_class::<RustTest>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();

