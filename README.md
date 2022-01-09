# Coll

_SHA256 Collision Finder_

![image](https://user-images.githubusercontent.com/6520433/148701593-0a0c92ae-51b7-4d1c-a2b5-3c57b78f23a7.png)

## What's This?

I was curious how long it might take to find a collision for an SHA256 hash created from an arbitrary file on my computer (a 20MB PDF in this case). I don't care about making a file that still works, or about matching the size of the existing file, simply about how long it would take to create a sequence of bytes that would result in the same hash.

This is a very naive implementation since it is only looking for uppercase letter sequences like "A", "B", ... "AA", "AB", etc.

## Future Goals

* expanded sequence, using all bytes from 0 to 254 (I believe 255 is an EOF signal) instead of just uppercase alphabet
* multi-threading
* improved visualization of progression
* write the collision content to a file

## About the Author

I've been a developer for a lot of years, but this is my first exposure to Rust. I figured this would be an interesting project to try with it.
