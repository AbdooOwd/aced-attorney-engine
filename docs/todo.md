# Todo

-   Add a visiblity system for textures.
    *   Maybe what I'll do is make a new struct `Texture` that takes 2 fields:
        `texture: Texture2D` & `visible: bool`.
        Maybe I'll add in the texture drawing logic (somewhere, or at least
        where it needs it) that if its visiblity is false, it ignores drawing.