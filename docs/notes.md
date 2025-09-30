# Notes

-   There are special values for the `text` field in `TextboxDataEntry`:
    *   `"shout:object"`: Displays the "Objection!" shouting.
-   It appears that updating a `Texture2D` with an `Image` that has
    a different size than the previous texture crashes the thing...
-   Nuh uh. Adding the `image` crate ain't worth it for just
    an app icon. It adds too much dependencies (which reach 1.6GiB).
    I think I'll just leave the default icon or code my own thing
    (cuz I only need to get the bytes of a picture's file.
    I don't need all the extra heavy stuff).
    *   After reflexions, the `image` crate **might** be useful.
        But only if I want to extract an image's bytes.
        Because if I only want to display it, `macroquad`
        does a wonderful job doing so. So I need an alternative
        to extract an image's bytes.
    *   I could just iterate over the image's file bytes, but
        for some reasons, PNG files leave the rest of the bytes
        (if they're supposed to represent transparent pixels)
        non-existent. I did think it was weird that a 16*16
        image only had ~700 bytes while the image was supposed
        to have around 1024 bytes.