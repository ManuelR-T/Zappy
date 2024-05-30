/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** init_router
*/

#include "json/json.h"
#include "logger.h"
#include "router/router.h"
#include "str.h"

static str_t *get_path(str_t const *name)
{
    str_t *path = str_snew("config/router/");

    str_sadd(path, name);
    str_scadd(path, ".json");
    return path;
}

static void load_routes(struct vec_json_t *routes, struct router *router)
{
    if (!routes)
        return logs(ERROR_LEVEL, "Failed to load routes\n");

    for (size_t i = 0; i < routes->size; i++) {
    }
    (void)router;
}

struct router *init_router(char const *file)
{
    struct router *router = vec_create_router(10);
    json_data_t *json = json_from_file(file);
    str_t *route_key = str_snew("routes");

    if (!json || !router)
        return NULL;
    load_routes(json_get_array(json, route_key), router);
    json_free(json);
    return router;
}
