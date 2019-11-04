#ifndef XPSIMPORT_HG
#define XPSIMPORT_HG

#ifndef XPS_STATIC_BUILD
#define XPS_API 
#else
#define XPS_API
#endif
#ifdef __cplusplus
extern "C"
{
#endif

typedef struct Vector3
{
	float x, y, z;
} Vector3;

typedef struct Vector2
{
	float x, y;
} Vector2;

typedef struct Color
{
	unsigned char x, y, z, w;
} Color;

typedef struct XPSData XPSData;

XPS_API XPSData *xps_load_model(const char *filename);

XPS_API void xps_delete_model(XPSData *model);

XPS_API int xps_get_mesh_count(XPSData *model);

XPS_API int xps_get_bone_count(XPSData *model);

XPS_API const char *xps_get_bone_name(XPSData *model, int index);

XPS_API short xps_get_bone_parent_id(XPSData *model, int index);

XPS_API Vector3 xps_get_bone_position(XPSData *model, int index);

XPS_API const char *xps_get_mesh_name(XPSData *model, int mesh_index);

XPS_API int xps_get_uv_layers(XPSData *model, int mesh_index);

XPS_API int xps_get_vertex_count(XPSData *model, int mesh_index);

XPS_API int xps_get_texture_count(XPSData *model, int mesh_index);

XPS_API int xps_get_vertex_id(XPSData *model, int mesh_index, int vertex_index);

XPS_API Vector3 xps_get_vertex_position(XPSData *model, int mesh_index, int vertex_index);

XPS_API Vector3 xps_get_vertex_normal(XPSData *model, int mesh_index, int vertex_index);

XPS_API Color xps_get_vertex_color(XPSData *model, int mesh_index, int vertex_index);

XPS_API Vector2 xps_get_vertex_uv(XPSData *model, int mesh_index, int vertex_index, int layer_id);

XPS_API int xps_get_vertex_bone_index(XPSData *model, int mesh_index, int vertex_index, int weight_id);

XPS_API float xps_get_vertex_bone_weight(XPSData *model, int mesh_index, int vertex_index, int weight_id);
#ifdef __cplusplus
}
#endif
#endif