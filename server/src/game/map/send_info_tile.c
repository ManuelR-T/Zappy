/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** display_info_map
*/

#include <stdbool.h>
#include <stdio.h>
#include <unistd.h>

#include "types/map.h"
#include "utils.h"

static void check_ressource(
    int fd,
    char const *name,
    size_t nb_ressource,
    bool *is_first
)
{
    if (nb_ressource) {
        dprintf(fd, "%s%s", (*is_first) ? "" : " ", name);
        *is_first = false;
    }
}

void send_info_tile(int fd, map_t *map, size_t y, size_t x)
{
    bool fst = true;

    if (y > map->y || x > map->x) {
        logger_error("Be careful coordinates (%d, %d)"
            "is out of range of map of size (%d, %d)\n", y, x, map->y, map->x);
        return;
    }
    check_ressource(fd, "player", map->arena[y][x].players->size != 0, &fst);
    check_ressource(fd, "food", map->arena[y][x].content.food, &fst);
    check_ressource(fd, "linemate",
    map->arena[y][x].content.linemate, &fst);
    check_ressource(fd, "deraumere",
        map->arena[y][x].content.deraumere, &fst);
    check_ressource(fd, "sibur", map->arena[y][x].content.sibur, &fst);
    check_ressource(fd, "mendiane",
        map->arena[y][x].content.mendiane, &fst);
    check_ressource(fd, "phiras", map->arena[y][x].content.phiras, &fst);
    check_ressource(fd, "thystame",
        map->arena[y][x].content.thystame, &fst);
}
