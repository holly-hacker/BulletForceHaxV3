# Tech stack

Bullet Force uses the [Unity](https://unity.com/) game engine, compiled to various platforms using [IL2CPP](https://docs.unity3d.com/6000.0/Documentation/Manual/scripting-backends-il2cpp.html) which transpiles the managed .NET assemblies the C# compiler produces into C++ code, which is then compiled to the target platform (being WebAssembly for WebGL, x86-64 for PC and ARM for Android and iOS).

For networking, Bullet Force uses Photon Unity Networking (PUN). This section of the book won't explain PUN in detail. As of 2025-04-23, Bullet Force uses `v4.0.29.11263` of the Photon OnPremise Plugins SDK.

Bullet Force also uses various other libraries. These may be listed at a later point.
<!-- TODO: list dependencies -->
