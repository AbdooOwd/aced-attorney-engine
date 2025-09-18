# Todo

-   Add a visiblity system for textures.
    *   Maybe what I'll do is make a new struct `Texture` that takes 2 fields:
        `texture: Texture2D` & `visible: bool`.
        Maybe I'll add in the texture drawing logic (somewhere, or at least
        where it needs it) that if its visiblity is false, it ignores drawing.
-   [!] I **REALLY** need an "Audio Manager" or something like that.
    I need to know what audio is playing, if it's done playing, and all that.