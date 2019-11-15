
# xpsimport
A rust library for parsing XPS/XnaLara meshes. This repository is used for making XNALaraUnity to work. Supports both binary and ASCII formats.

### Usage
```
use xpsimport::loader;  
use xpsimport::bone_naming;  
fn main() {  
  if let Ok(model) = loader::open("mesh.xps",   
   bone_naming::BoneNaming::Default, true, true) {  
    for mesh in model.meshes {  
      println!("{}", mesh.name.to_str().unwrap());  
    }  
  }  
}
```
A C interface is provided along with cmake config to be easily integrated with your cpp project. Use this cmake function:

``target_link_xpsimport_library(target_name)``

C API usage:
```
#include  <stdio.h>
#include  <xpsimport.h>

int  main() {
  XPSData *mdl =  xps_load_model("C:/test/carl.mesh");
  int error =  xps_get_error(mdl);
  const  char* mesh_name =  xps_get_mesh_name(mdl, 0);
  printf("%s", mesh_name);
  printf("a%d", error);
  return  0;
}
```
A full info on available API functions is provided in [xpsimport.h](include/xpsimport.h) 
### License 
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
### References

[XPS](http://www.core-design.com/community_xps.html) by core-design
[XNALaraMesh](https://github.com/johnzero7/XNALaraMesh) by johnzero7
