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

	static const unsigned char XPS_ERROR_STREAM_NOT_OPENED = 0;
	static const unsigned char XPS_ERROR_INVALID_HEADER = 1;
	static const unsigned char XPS_ERROR_UNKNOWN = 2;
	static const unsigned char XPS_ERROR_FILE_NOT_LOADED = 3;
	static const unsigned char XPS_ERROR_PATH_GET_PARENT = 4;
	static const unsigned char XPS_ERROR_PATH_TO_STR = 5;
	static const unsigned char XPS_ERROR_MESH_READ_ASCII = 6;
	static const unsigned char XPS_ERROR_MESH_READ_BIN = 7;
	static const unsigned char XPS_ERROR_NONE = 8;

	XPS_API XPSData *xps_load_model(const char *filename);

	XPS_API unsigned char xps_get_error(XPSData *model);

	XPS_API void xps_delete_model(XPSData *model);

	XPS_API int xps_get_mesh_count(XPSData *model);

	XPS_API int xps_get_mesh_index_count(XPSData *model, int mesh_index);

	XPS_API int xps_get_mesh_index(XPSData *model, int mesh_index, int index_num);

	XPS_API int xps_get_bone_count(XPSData *model);

	XPS_API int xps_get_texture_count(XPSData *model, int mesh_index);
	
	XPS_API int xps_get_texture_id(XPSData *model, int mesh_index, int texture_index);
	
	XPS_API int xps_get_texture_filename(XPSData *model, int mesh_index, int texture_index);
	
	XPS_API int xps_get_texture_uv_layer(XPSData *model, int mesh_index, int texture_index);

	XPS_API const char *xps_get_bone_name(XPSData *model, int index);

	XPS_API int xps_get_bone_parent_id(XPSData *model, int index);

	XPS_API Vector3 xps_get_bone_position(XPSData *model, int index);

	XPS_API const char *xps_get_mesh_name(XPSData *model, int mesh_index);

	XPS_API int xps_get_uv_layers(XPSData *model, int mesh_index);

	XPS_API int xps_get_vertex_count(XPSData *model, int mesh_index);

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