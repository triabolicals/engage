use unity::engine::{Vector3, Vector2, Material};
use unity::il2cpp::object::Array;
use unity::prelude::*;
use unity::system::SystemType;

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Space {
    World = 0,
    Local = 1,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Bounds {
    pub center: Vector3<f32>,
    pub extent: Vector3<f32>,
}
impl Bounds {
    pub fn new() -> Self {
        Self {
            center: Vector3::new(0.0, 0.0, 0.0),
            extent: Vector3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn get_size(&self) -> Vector3<f32> { Vector3::new(2.0*self.extent.x, 2.0*self.extent.y, 2.0*self.extent.z) }
}

#[repr(C)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}
impl Matrix4x4 {
    pub fn new() -> Self { Matrix4x4 { m: [[0.0; 4]; 4] } }
}

#[unity::class("UnityEngine", "Quaternion")] pub struct QuaternionClass {}
#[unity::class("UnityEngine", "Vector3")] pub struct Vector3Class {}
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3f {
    pub fn class() -> &'static Il2CppClass { Vector3Class::class() }
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}



impl Quaternion {
    pub fn class() -> &'static Il2CppClass { QuaternionClass::class() }
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Quaternion { x, y, z, w } }
    pub fn look_rotation_z_axis(forward: &Vector3<f32>) -> Quaternion {
        let up = Vector3::new(0.0, 1.0, 0.0);
        Self::look_rotation(forward, &up)
    }
    pub fn look_rotation(forward: &Vector3<f32>, upwards: &Vector3<f32>) -> Quaternion {
        let mut ret = Quaternion::default();
        Self::look_rotation_injected(forward, upwards, &mut ret);
        ret
    }
    pub fn from_to_rotation(from_direction: &Vector3<f32>, to_direction: &Vector3<f32>) -> Quaternion {
        let mut ret = Quaternion::default();
        Self::from_to_rotation_injected(from_direction, to_direction, &mut ret);
        ret
    }
    pub fn inverse(rotation: &Quaternion) -> Quaternion {
        let mut ret = Quaternion::default();
        Self::inverse_injected(rotation, &mut ret);
        ret
    }
    pub fn get_euler_angles(&self) -> Vector3<f32> {
        let mut ret = Vector3::new(0.0, 0.0, 0.0);
        Self::internal_to_euler_rad_injected(self, &mut ret);
        ret
    }
    #[unity::class_method(43, QuaternionClass)] pub fn look_rotation_injected(forward: &Vector3<f32>, upwards: &Vector3<f32>, ret: &mut Quaternion); // Offset: 0x32F9C90 Flags: 0
    #[unity::class_method(34, QuaternionClass)] pub fn from_to_rotation_injected(from_direction: &Vector3<f32>, to_direction: &Vector3<f32>, ret: &mut Quaternion); // Offset: 0x32F9530 Flags: 0
    #[unity::class_method(35, QuaternionClass)] pub fn inverse_injected(rotation: &Quaternion, ret: &mut Quaternion); // Offset: 0x32F95F0 Flags: 0
    #[unity::class_method(40, QuaternionClass)] pub fn internal_to_euler_rad_injected(rotation: &Quaternion, ret: &mut Vector3<f32>); // Offset: 0x32F9A30 Flags: 0
}

// UnityEngine.AnimationEvent$$get_stringParameter	7103ead6c0	System_String_o * UnityEngine.AnimationEvent$$get_stringParameter(UnityEngine_AnimationEvent_o * __this, MethodInfo * method)	8
#[unity::from_offset("UnityEngine", "AnimationEvent", "get_stringParameter")]
pub fn animationevent_get_stringparameter(this: *const u8, method_info: OptionalMethod, ) -> Option<&'static Il2CppString>;

#[unity::class("UnityEngine", "Object")]
pub struct Object { pub cache_ptr: *const u8, }

#[unity::class("UnityEngine", "MonoBehavior")]
pub struct MonoBehavior { ptr: *const u8, }

pub trait UnityObject: Il2CppClassData + Sized {
    fn find_objects(include_inactive: bool) -> &'static Array<&'static mut Self> {
        let find_method = Object::class().get_methods()[25];
        let find = unsafe { std::mem::transmute::<_, fn(&SystemType, bool, &MethodInfo) -> &'static Array<&'static mut Self>>(find_method.method_ptr) };
        find(Self::class().get_system_type(), include_inactive, find_method)
    }
    fn find_object(include_inactive: bool) -> Option<&'static mut Self> {
        let find_method = Object::class().get_methods()[40];
        let find = unsafe { std::mem::transmute::<_, fn(&SystemType, bool, &MethodInfo) -> Option<&'static mut Self>>(find_method.method_ptr) };
        find(Self::class().get_system_type(), include_inactive, find_method)
    }
    #[unity::class_method(0, Object)] fn get_instance_id(&self) -> i32; // Offset: 0x32EDE90 Flags: 0
    #[unity::class_method(2, Object)] fn equals<Obj>(&self, other: &Obj) -> bool where Obj: UnityObject; // Offset: 0x32EE040 Flags: 0
    #[unity::class_method(3, Object)] fn op_implicit(&self) -> bool; // Offset: 0x32EE320 Flags: 0
    #[unity::class_method(8, Object)] fn get_name(&self) -> &'static Il2CppString; // Offset: 0x32E8E90 Flags: 0
    #[unity::class_method(9, Object)] fn set_name(&self, value: &Il2CppString); // Offset: 0x32EE5F0 Flags: 0
    #[unity::class_method(21, Object)] fn destroy(&self); // Offset: 0x32EF640 Flags: 0
    #[unity::class_method(42, Object)] fn equal<Obj>(&self, y: &Obj) -> bool where Obj: UnityObject; // Offset: 0x32E8A10 Flags: 0
    #[unity::class_method(43, Object)] fn not_equal<Obj>(&self, y: &Obj) -> bool where Obj: UnityObject; // Offset: 0x32EFED0 Flags: 0 }
}



#[unity::class("UnityEngine", "GameObject")]
pub struct GameObject { pub cache_ptr: *const u8, }

impl GameObject {
    pub fn is_null(&self) -> bool { !self.op_implicit() }
    pub fn find<'a>(name: impl Into<&'a Il2CppString>) -> Option<&'static GameObject> { Self::find_(name.into()) }

    pub fn get_component_by_type<T: UnityComponent>(&self) -> Option<&'static mut T> {
        self.get_component_(T::class().get_system_type())
    }
    pub fn get_component_in_parent<T: UnityComponent>(&self, include_inactive: bool) -> Option<&'static mut T> {
        self.get_component_in_parent_(T::class().get_system_type(), include_inactive)
    }
    pub fn get_component_in_children<T: UnityComponent>(&self, include_inactive: bool) -> Option<&'static mut T> {
        self.get_component_in_children_(T::class().get_system_type(), include_inactive)
    }
    pub fn get_components<T: UnityComponent>(&self) -> &'static Array<&'static mut T> {
        self.get_components_(T::class().get_system_type())
    }
    pub fn get_components_in_children<T: UnityComponent>(&self, include_inactive: bool) -> &'static mut Array<&'static mut T> {
        self.get_components_in_children_(T::class().get_system_type(), include_inactive)
    }
    pub fn get_components_in_parent<T: UnityComponent>(&self, include_inactive: bool) -> &'static Array<&'static mut T>{
        self.get_components_in_parent_(T::class().get_system_type(), include_inactive)
    }

    #[unity::class_method(2)] fn get_component_<T>(&self, ty: &SystemType) -> Option<&'static mut T> where T: UnityComponent;
    #[unity::class_method(6)] fn get_component_in_children_<T>(&self, ty: &SystemType, include_inactive: bool) -> Option<&'static mut T> where T: UnityComponent;
    #[unity::class_method(10)] fn get_component_in_parent_<T>(&self, ty: &SystemType, include_inactive: bool) -> Option<&'static mut T> where T: UnityComponent;
    #[unity::class_method(15)] fn get_components_<T>(&self, ty: &SystemType) -> &'static Array<&'static mut T> where T: UnityComponent;
    #[unity::class_method(20)]
    fn get_components_in_children_<T>(&self, component_type: &SystemType, include_inactive: bool) -> &'static mut Array<&'static mut T> where T: UnityComponent;
    #[unity::class_method(21, generic)]
    pub fn get_components_in_children2<T>(&self, include_inactive: bool) -> &'static Array<&'static T> where T: Il2CppClassData; // Offset: -1 Flags: 1
    #[unity::class_method(26)]
    fn get_components_in_parent_<T>(&self, component_type: &SystemType, include_inactive: bool) -> &'static Array<&'static mut T> where T: UnityComponent;

    #[unity::class_method(42)] pub fn get_transform(&self) -> &'static Transform;
    #[unity::class_method(43)] pub fn get_layer(&self) -> i32;
    #[unity::class_method(44)] pub fn set_layer(&self, layer: i32);
    #[unity::class_method(45)] pub fn get_active(&self) -> bool;
    #[unity::class_method(46)] pub fn set_active(&self, active: bool);
    #[unity::class_method(47)] pub fn set_active2(&self, active: bool);
    #[unity::class_method(50)] pub fn set_active_recursively(&self, active: bool);
    #[unity::class_method(72)] pub fn find_(name: &Il2CppString) -> Option<&'static GameObject>;

}
impl UnityObject for GameObject {}

#[unity::class("UnityEngine", "Camera")]
pub struct Camera {}
impl Camera {
    #[unity::class_method(5)] pub fn get_fov(&self) -> f32; // Offset: 0x2C39310 Flags: 0
    #[unity::class_method(6)] pub fn set_fov(&self, value: f32); // Offset: 0x2C39360 Flags: 0
    #[unity::class_method(32)] pub fn get_aspect(&self) -> f32; // Offset: 0x2C39C40 Flags: 0
    #[unity::class_method(137)] pub fn get_main() -> Option<&'static Camera>; // Offset: 0x2C3D2B0 Flags: 0
}

impl UnityObject for Camera {}
impl UnityComponent for Camera {}

#[unity::class("UnityEngine", "Renderer")] pub struct Renderer {}
impl Renderer {
    pub fn get_bounds(&self) -> Bounds {
        let mut bounds = Bounds::new();
        Self::get_bounds_injected(self, &mut bounds);
        bounds
    }
    #[unity::class_method(37)] pub fn get_bounds_injected(&self, ret: &mut Bounds); // Offset: 0x2F880B0 Flags: 0
    #[unity::class_method(2)] pub fn get_material(&self) -> &'static Material; // Offset: 0x2F881B0 Flags: 0
}
impl UnityComponent for Renderer {}

#[unity::class("UnityEngine", "Animator")] pub struct Animator {}
#[unity::class("UnityEngine", "Component")] pub struct Component {}
#[unity::class("UnityEngine", "Transform")] pub struct Transform {}
#[unity::class("UnityEngine", "RectTransform")] pub struct RectTransform {}
#[unity::class("UnityEngine", "RenderTexture")] pub struct RenderTexture {}
#[unity::class("UnityEngine", "SkinnedMeshRenderer")] pub struct SkinnedMeshRenderer {}
#[unity::class("UTJ", "SpringBone")] pub struct SpringBone{}
#[unity::class("UTJ.Jobs", "SpringJobManager")] pub struct SpringJobManager {}

impl UnityComponent for SpringJobManager {}
impl UnityObject for SpringJobManager {}

impl Animator {
    pub fn get_bool<'a>(&self, name: impl Into<&'a Il2CppString>) -> bool { self.get_bool_(name.into()) }
    pub fn set_bool<'a>(&self, name: impl Into<&'a Il2CppString>, value: bool) { self.set_bool_(name.into(), value) }
    pub fn play<'a>(&self, name: impl Into<&'a Il2CppString>) { self.play_(name.into()) }
    pub fn string_to_hash<'a>(name: impl Into<&'a Il2CppString>) -> i32 { Self::string_to_hash_(name.into()) }
    #[unity::class_method("Play")] fn play_me(&self, time: i32) -> bool;
    #[unity::class_method(12)] pub fn get_bool_(&self, name: &Il2CppString) -> bool; // Offset: 0x3EB0B30 Flags: 0
    #[unity::class_method(14)] pub fn set_bool_(&self, name: &Il2CppString, value: bool); // Offset: 0x3EB0C70 Flags: 0
    #[unity::class_method(157)] pub fn play_(&self, state_name: &Il2CppString); // Offset: 0x3EB5770 Flags: 0
    #[unity::class_method(190)] pub fn string_to_hash_(name: &Il2CppString) -> i32; // Offset: 0x3EB4A60 Flags: 0
}
impl UnityComponent for Animator {}
impl UnityObject for Animator {}

pub trait UnityComponent: Il2CppClassData + Sized {
    fn get_component<T: UnityComponent>(&self) -> Option<&'static mut T> { self.get_component_(T::class().get_system_type()) }
    fn get_component_in_children<T: UnityComponent>(&self, include_inactive: bool) -> Option<&'static T>{
        self.get_component_in_children_(T::class().get_system_type(), include_inactive)
    }
    fn get_component_in_parent<T: UnityComponent>(&self) -> Option<&'static T> {
        self.get_component_in_parent_(T::class().get_system_type())
    }
    fn destroy_game_object(&self) { if let Some(go) = self.get_game_object() { go.destroy(); } }
    #[unity::class_method(9, Component, generic)]
    fn get_components_in_children_gen<T>(&self, include_inactive: bool) -> &'static Array<&'static mut T> where T: Il2CppClassData;

    #[unity::class_method(0, Component)] fn get_transform(&self) -> &'static Transform;
    #[unity::class_method(1, Component)] fn get_game_object(&self) -> Option<&'static GameObject>;
    #[unity::class_method(2, Component)] fn get_component_<T>(&self, ty: &SystemType) -> Option<&'static mut T> where T: UnityComponent; // Offset: 0x2C464E0 Flags: 0
    #[unity::class_method(6, Component)] fn get_component_in_children_<T>(&self, t: &SystemType, include_inactive: bool) -> Option<&'static T>  where T: UnityComponent; // Offset: 0x2C46610 Flags: 0

    #[unity::class_method(13, Component)] fn get_component_in_parent_<T>(&self, t: &SystemType) -> Option<&'static T> where T: UnityComponent; //Offset: 0x2C46700 Flags: 0
}
impl UnityComponent for Component {}
impl UnityObject for Component {}

impl Component {
    #[unity::class_method(1)]  pub fn get_go<T>(obj: &T) -> Option<&'static GameObject> where T: Il2CppClassData;
}


// Trait for both UnityEngine.Transform and UnityEngine.RectTransform
pub trait UnityTransform: Il2CppClassData + Sized {
    fn find<'a>(&self, name: impl Into<&'a Il2CppString>) -> Option<&'static Transform> { self.find_(name.into()) }

    fn set_position(&self, v: Vector3<f32>) { Self::set_position_injected(self, &v) }
    fn set_rotation(&self, v: Quaternion) { Self::set_rotation_injected(self, &v) }
    fn set_euler_angles(&self, v: Vector3<f32>) { unsafe { transform_set_euler_angles(self, v, None) }}

    fn set_local_position(&self, v: Vector3<f32>){ Self::set_local_position_injected(self, &v) }
    fn set_local_rotation(&self, v: Quaternion){ Self::set_local_rotation_injected(self, &v) }
    fn set_local_scale(&self, v: Vector3<f32>){ Self::set_local_scale_injected(self, &v) }


    fn get_position(&self) -> Vector3<f32> {
        let mut ret = Vector3::<f32>::new(0.0, 0.0, 0.0);
        Self::get_position_injected(self, &mut ret);
        ret
    }
    fn get_local_position(&self) -> Vector3<f32> {
        let mut ret = Vector3::<f32>::new(0.0, 0.0, 0.0);
        Self::get_local_position_injected(self, &mut ret);
        ret
    }
    fn get_local_rotation(&self) -> Quaternion {
        let mut ret = Quaternion::default();
        Self::get_local_rotation_injected(self, &mut ret);
        ret
    }
    fn get_rotation(&self) -> Quaternion {
        let mut ret = Quaternion::default();
        Self::get_rotation_injected(self, &mut ret);
        ret
    }
    fn get_local_scale(&self) -> Vector3<f32> {
        let mut ret = Vector3::<f32>::new(0.0, 0.0, 0.0);
        Self::get_local_scale_injected(self, &mut ret);
        ret
    }
    fn get_euler_angles(&self) -> Vector3<f32> { self.get_rotation().get_euler_angles() }

    fn get_right(&self) -> Vector3<f32> { unsafe { transform_get_right(self, None) } }
    fn get_forward(&self) -> Vector3<f32> { unsafe { transform_get_forward(self, None) } }
    fn get_up(&self) -> Vector3<f32> { unsafe { transform_get_up(self, None) } }

    //fn get_position(&self) -> Vector3<f32> { unsafe { transform_position(self, None) } }
    // fn set_position(&self, pos: Vector3<f32>) { unsafe { transform_set_position(self, pos, None) }}
    // fn get_local_rotation(&self) -> Vector4<f32> { unsafe { transform_get_local_rotation(self, None) } }
    // fn set_local_rotation(&self, v: Vector4<f32>) { unsafe { transform_set_local_rotation(self, v, None )}}
    // fn set_local_scale(&self, s: Vector3<f32>) { unsafe { transform_set_local_scale(self, s, None) }}
    // fn get_local_scale(&self) -> Vector3<f32> { unsafe { transform_get_local_scale(self, None) } }
    // fn get_rotation(&self) -> Vector4<f32> { unsafe { transform_get_rotation(self, None) } }
    // fn set_rotation(&self, v: Vector4<f32>) { unsafe { transform_set_rotation(self, v, None) }}
    // fn get_euler_angle(&self) -> Vector3<f32> { unsafe { transform_get_euler_angles(self, None)}}

    #[unity::class_method(28, Transform)] fn get_parent(&self) -> Option<&'static Transform>; // Offset: 0x37909F0 Flags: 0
    #[unity::class_method(33, Transform)] fn set_parent(&self, parent: Option<&Transform>); // Offset: 0x3790C30 Flags: 0
    #[unity::class_method(34, Transform)] fn set_parent2(&self, parent: Option<&Transform>, world_position_stays: bool); // Offset: 0x3790C90 Flags: 0
    #[unity::class_method(41, Transform)] fn translate_local(&self, x: f32, y: f32, z: f32); // Offset: 0x3791140 Flags: 0
    #[unity::class_method(47, Transform)] fn rotate_local(&self, x_angle: f32, y_angle: f32, z_angle: f32); // Offset: 0x3791530 Flags: 0

    #[unity::class_method(53, Transform)] fn look_at_transform(&self, target: &Transform); // Offset: 0x3791B10 Flags: 0
    #[unity::class_method(71, Transform)] fn get_child_count(&self) -> i32; // Offset: 0x37925A0 Flags: 0
    #[unity::class_method(72, Transform)] fn detach_children(self); // Offset: 0x37925F0 Flags: 0
    #[unity::class_method(78, Transform)] fn find_relative_transform_with_path(transform: &Transform, path: &Il2CppString, is_active_only: bool) -> Option<&'static Transform>; // Offset: 0x37927E0 Flags: 0

    #[unity::class_method(79, Transform)] fn find_(&self, n: &Il2CppString) -> Option<&'static Transform>; // Offset: 0x3792840 Flags: 0
    #[unity::class_method(85, Transform)] fn find_child_(&self, n: &Il2CppString) -> Option<&'static Transform>; // Offset: 0x3792AD0 Flags: 0
    #[unity::class_method(89, Transform)] fn get_child(&self, index: i32) -> Option<&'static Transform>; // Offset: 0x3792D40 Flags: 0

    #[unity::class_method(98, Transform)] fn get_position_injected(&self, ret: &mut Vector3<f32>); // Offset: 0x378F8F0 Flags: 0
    #[unity::class_method(99, Transform)] fn set_position_injected(&self, value: &Vector3<f32>); // Offset: 0x378F9A0 Flags: 0
    #[unity::class_method(100, Transform)] fn get_local_position_injected(&self, ret: &mut Vector3<f32>); // Offset: 0x378FA50 Flags: 0
    #[unity::class_method(101, Transform)] fn set_local_position_injected(&self, value: &Vector3<f32>); // Offset: 0x378FB00 Flags: 0
    #[unity::class_method(105, Transform)] fn get_rotation_injected(&self, ret: &mut Quaternion); // Offset: 0x3790610 Flags: 0
    #[unity::class_method(106, Transform)] fn set_rotation_injected(&self, value: &Quaternion); // Offset: 0x3790660 Flags: 0
    #[unity::class_method(107, Transform)] fn get_local_rotation_injected(&self, ret: &mut Quaternion); // Offset: 0x37906B0 Flags: 0
    #[unity::class_method(108, Transform)] fn set_local_rotation_injected(&self, value: &Quaternion); // Offset: 0x3790700 Flags: 0
    #[unity::class_method(109, Transform)] fn get_local_scale_injected(&self, ret: &mut Vector3<f32>); // Offset: 0x37908F0 Flags: 0
    #[unity::class_method(110, Transform)] fn set_local_scale_injected(&self, value: &Vector3<f32>); // Offset: 0x37909A0 Flags: 0
    #[unity::class_method(111, Transform)] fn get_world_to_local_matrix_injected(&self, ret: &mut Matrix4x4); // Offset: 0x3790D80 Flags: 0
    #[unity::class_method(112, Transform)] fn get_local_to_world_matrix_injected(&self, ret: &mut Matrix4x4); // Offset: 0x3790E60 Flags: 0
}
impl UnityTransform for Transform {}
impl UnityComponent for Transform {}
impl UnityObject for Transform {}

impl RectTransform {
    pub fn get_size_delta(&self) -> Vector2<f32> { unsafe { rect_transform_get_size_delta(self, None) } }
    pub fn set_size_delta(&self, v: Vector2<f32>) { unsafe { rect_transform_set_size_delta(self, v, None) } }
    #[unity::class_method(33)] pub fn get_anchor_min_injected(&self, ret: &mut Vector2<f32>); // Offset: 0x2F7C500 Flags: 0
    #[unity::class_method(34)] pub fn set_anchor_min_injected(&self, value: &Vector2<f32>); // Offset: 0x2F7C5B0 Flags: 0
    #[unity::class_method(35)] pub fn get_anchor_max_injected(&self, ret: &mut Vector2<f32>); // Offset: 0x2F7C660 Flags: 0
    #[unity::class_method(36)] pub fn set_anchor_max_injected(&self, value: &Vector2<f32>); // Offset: 0x2F7C710 Flags: 0
    #[unity::class_method(37)] pub fn get_anchored_position_injected(&self, ret: &mut Vector2<f32>); // Offset: 0x2F7C7C0 Flags: 0
    #[unity::class_method(38)] pub fn set_anchored_position_injected(&self, value: Vector2<f32>); // Offset: 0x2F7C870 Flags: 0
    #[unity::class_method(39)] pub fn get_size_delta_injected(&self, ret: &mut Vector2<f32>); // Offset: 0x2F7C920 Flags: 0
    #[unity::class_method(40)] pub fn set_size_delta_injected(&self, value: &Vector2<f32>); // Offset: 0x2F7C9D0 Flags: 0
    #[unity::class_method(41)] pub fn get_pivot_injected(&self, ret: &Vector2<f32>); // Offset: 0x2F7CA80 Flags: 0
    #[unity::class_method(42)] pub fn set_pivot_injected(&self, value: &Vector2<f32>); // Offset: 0x2F7CB30 Flags: 0
}

impl UnityTransform for RectTransform {}
impl UnityComponent for RectTransform {}
impl UnityObject for RectTransform {}

impl RenderTexture {
    #[unity::class_method(0)] pub fn get_width(&self) -> i32; // Offset: 0x2F84A20 Flags: 0
    #[unity::class_method(1)] pub fn set_width(&self, value: i32); // Offset: 0x2F84A70 Flags: 0
    #[unity::class_method(2)] pub fn get_height(&self) -> i32; // Offset: 0x2F84AC0 Flags: 0
    #[unity::class_method(3)] pub fn set_height(&self, value: i32); // Offset: 0x2F84B10 Flags: 0
    #[unity::class_method(51)] pub fn create(&self) -> bool; // Offset: 0x2F85AE0 Flags: 0
    #[unity::class_method(52)] pub fn release(&self); // Offset: 0x2F85B30 Flags: 0
}
impl UnityObject for RenderTexture {}

impl SkinnedMeshRenderer {
    #[unity::class_method(28, Renderer)] pub fn get_materials(&self) -> &'static Array<&'static Material>; // Offset: 0x2F88A90 Flags: 0
    #[unity::class_method(29, Renderer)] pub fn set_materials(&self, value: &Array<&'static Material>); // Offset: 0x2F88AE0 Flags: 0
    #[unity::class_method(2, Renderer)] pub fn get_material(&self) -> &'static Material; // Offset: 0x2F881B0 Flags: 0
    #[unity::class_method(5, Renderer)] pub fn get_material_array(&self) -> &'static Array<&'static Material>; // Offset: 0x2F882A0 Flags: 0
    #[unity::class_method(6, Renderer)] pub fn set_material_array(&self, m: &Array<&Material>); // Offset: 0x2F882F0 Flags: 0
    #[unity::class_method(10, Renderer)] pub fn set_enabled(&self, value: bool); // Offset: 0x2F88430 Flags: 0
    #[unity::class_method(34, Renderer)] pub fn get_shared_materials(&self) -> &'static Array<&'static Material>; // Offset: 0x2F88C70 Flags: 0
    #[unity::class_method(35, Renderer)] pub fn set_shared_materials(&self, value: &Array<&Material>); // Offset: 0x2F88CC0 Flags: 0
    #[unity::class_method(4)] pub fn get_force_matrix_recalculation_per_render(&self) -> bool; // Offset: 0x2F93E10 Flags: 0
    #[unity::class_method(5)] pub fn set_force_matrix_recalculation_per_render(&self, value: bool); // Offset: 0x2F93E60 Flags: 0
    #[unity::class_method(6)] pub fn get_root_bone(&self) -> &'static Transform; // Offset: 0x2F93EB0 Flags: 0
    #[unity::class_method(7)] pub fn set_root_bone(&self, value: &Transform); // Offset: 0x2F93F00 Flags: 0
    #[unity::class_method(8)] pub fn get_bones(&self) -> &'static Array<&'static Transform>; // Offset: 0x2F93F50 Flags: 0
    #[unity::class_method(9)] pub fn set_bones(&self, value: &Array<&'static Transform>); // Offset: 0x2F93FA0 Flags: 00
    #[unity::class_method(10)] pub fn get_shared_mesh(&self) -> &'static Mesh; // Offset: 0x2F93FF0 Flags: 0
    #[unity::class_method(11)] pub fn set_shared_mesh(&self, value: &Mesh); // Offset: 0x2F94040 Flags: 0
}
impl UnityComponent for SkinnedMeshRenderer {}
impl UnityObject for SkinnedMeshRenderer {}

#[unity::class("UnityEngine", "Mesh")]
pub struct Mesh {}

impl UnityComponent for SpringBone {}
impl UnityObject for SpringBone {}

/*
#[skyline::from_offset(0x1e2b8d0)] fn go_get_com_in_children<C: UnityComponent>(this: &GameObject, method_info: OptionalMethod) -> &'static mut Array<&'static mut C>;
#[skyline::from_offset(0x2c4e0d0)] fn go_get_component_in_children(this: &GameObject, system_type: &SystemType, include_inactive: bool, optional_method: OptionalMethod) -> &'static Array<&'static Component>;
#[skyline::from_offset(0x2C4DDF0)] fn go_get_comp_in_parent(this: &GameObject, ty: &SystemType, include_inactive: bool, method_info: OptionalMethod) -> Option<&'static Component>;
#[skyline::from_offset(0x2c46560)] fn go_get_component_by_type(this: &GameObject, ty: &SystemType, method_info: OptionalMethod) -> Option<&'static Component>;
#[skyline::from_offset(0x2c4dee0)] fn go_get_components(this: &GameObject, ty: &SystemType, method_info: OptionalMethod) -> &'static Array<&'static Component>;
#[skyline::from_offset(0x2c4e250)] fn go_get_component_in_parents(this: &GameObject, ty: &SystemType, inactive: bool, method_info: OptionalMethod) -> &'static Array<&'static Component>;
#[skyline::from_offset(0x3790c30)] fn transform_set_parent<S: UnityTransform, T: UnityTransform>(this: &S, parent: &T, optional_method: OptionalMethod);
#[skyline::from_offset(0x3790be0)] fn transform_get_parent<S: UnityTransform>(this: &S, optional_method: OptionalMethod) -> &'static Transform;
#[skyline::from_offset(0x3792840)] fn transform_find<S: UnityTransform, T: UnityTransform>(this: &S, n: &Il2CppString, optional_method: OptionalMethod) -> Option<&'static mut T>;
#[skyline::from_offset(0x378F890)] fn transform_position<T: UnityTransform>(this: &T, method_info: OptionalMethod)-> Vector3<f32>;
#[skyline::from_offset(0x378f940)] fn transform_set_position<T: UnityTransform>(this: &T, v: Vector3<f32>, method_info: OptionalMethod);
#[skyline::from_offset(0x37912e0)] fn transform_rotation(this: &Transform, x:f32, y:f32, z:f32, rel_to: i32, method_info: OptionalMethod);
#[skyline::from_offset(0x378ffc0)] fn transform_get_local_rotation<T: UnityTransform>(this: &T,optional_method: OptionalMethod)-> Vector4<f32>;
#[skyline::from_offset(0x378FE20)] fn transform_get_rotation<T: UnityTransform>(this: &T,optional_method: OptionalMethod)-> Vector4<f32>;
#[skyline::from_offset(0x3790080)] fn transform_set_local_rotation<T: UnityTransform>(this: &T, v: Vector4<f32>, optional_method: OptionalMethod);
*/
#[skyline::from_offset(0x378FDA0)] fn transform_get_euler_angles<T: UnityTransform>(this: &T, optional_method: OptionalMethod) -> Vector3<f32>; // Offset: 0x378FDA0 Flags: 0
#[skyline::from_offset(0x378fe80)] fn transform_set_euler_angles<T: UnityTransform>(this: &T, v: Vector3<f32>, optional_method: OptionalMethod);
#[skyline::from_offset(0x378FEE0)] fn transform_set_rotation<T: UnityTransform>(this: &T, v: Quaternion, optional_method: OptionalMethod);
#[skyline::from_offset(0x3791820)] fn rotate_around_point<T: UnityTransform>(this: &T, point: Vector3<f32>, axis: Vector3<f32>, angle: f32, method_info: OptionalMethod);
#[skyline::from_offset(0x37904A0)] fn transform_get_forward<T: UnityTransform>(this: &T, optional_method: OptionalMethod)-> Vector3<f32>;
#[skyline::from_offset(0x37902C0)] fn transform_get_up<T: UnityTransform>(this: &T, optional_method: OptionalMethod) -> Vector3<f32>; // Offset: 0x3
#[skyline::from_offset(0x37900E0)] fn transform_get_right<T: UnityTransform>(this: &T, optional_method: OptionalMethod) -> Vector3<f32>; // Offset: 0x3
/*
#[skyline::from_offset(0x3790940)] fn transform_set_local_scale<T: UnityTransform>(this: &T, v: Vector3<f32>, method_info: OptionalMethod);
#[skyline::from_offset(0x3790890)] fn transform_get_local_scale<T: UnityTransform>(this: &T,optional_method: OptionalMethod)-> Vector3<f32>;
 */

#[skyline::from_offset(0x2F7C8C0)] fn rect_transform_get_size_delta(this: &RectTransform, optional_method: OptionalMethod) -> Vector2<f32>;
#[skyline::from_offset(0x2F7C970)] fn rect_transform_set_size_delta(this: &RectTransform, v: Vector2<f32>, optional_method: OptionalMethod);

/*
#[skyline::from_offset(0x2c46610)] fn unity_component_get_from_children<S: UnityComponent, T: UnityComponent>(this: &S, ty: &SystemType, include_inactive: bool, method_info: OptionalMethod) -> Option<&'static mut T>;
#[skyline::from_offset(0x2c46700)] fn unity_component_get_from_parent<S: UnityComponent, T: UnityComponent>(this: &S, ty: &SystemType, method_info: OptionalMethod) -> Option<&'static mut T>;
#[skyline::from_offset(0x2c464e0)] fn unity_component_get<S: UnityComponent, T: UnityComponent>(this: &S, ty: &SystemType, method_info: OptionalMethod) -> Option<&'static mut T>;

#[skyline::from_offset(0x1e11990)] fn get_component_in_children_comp<T: UnityComponent, S: UnityComponent>(this: &S, method_info: &MethodInfo) -> &'static Array<&'static T>;

#[skyline::from_offset(0x32f9c20)] fn quaternion_look_rot_yz(z_axis: Vector3<f32>, y_axis: Vector3<f32>, optional_method: OptionalMethod) -> Quaternion;

 */
