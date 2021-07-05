# Notes

## Scratch

- A tile pool would be a cool idea. Like, unify the map and entity renders and make a flywheel for render systems.
- Lighting system 
```rust
    /// When paired with a Point, renders a brightness adjustment
    pub struct LightSource {
        // Brightness adjust will affect all render layers below z
        z: u32,
        // The radius of the "solid" light.
        r_solid: u32,
        // The radius of the "diffuse" light.
        r_diffuse: u32 
        // The intensity of the adjustment. Should be in [-1, 1]
        a: f64,
        // The shape to project
        shape: LightShape
        // Additional render options
        opts: Vec<LightOption>
    }

    /// The shape of the brightness adjustment
    pub enum LightShape {
        // 
        Point {}
        // Imagine a T, where the bottom of the T starts at point, the height is `r`, and the top is `width`
        Ray { angle: i32, width: u32 }
    }


    // Which of these 2 patterns to use?
    pub enum LightOption {
        Quiver { intensity: f64 }
    }

    pub trait LightOption {
        fn ApplyLightOption(&mut LightSource)
    }
```