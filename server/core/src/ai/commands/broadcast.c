/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** handle_broadcast
*/

#include <float.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <sys/socket.h>
#include <sys/types.h>

#include "core/client.h"
#include "core/router/route.h"
#include "core/types/ai.h"
#include "core/types/client.h"
#include "core/types/game.h"
#include "core/types/map.h"
#include "core/types/position.h"
#include "str.h"
#include "utils.h"

static bool valid_client(client_t *to_check, client_t *banned)
{
    return to_check->fd != 0 && to_check->ai->id != banned->ai->id;
}

static void get_starting_pos(
    pos_t *start_dest,
    pos_t *orig_pos,
    enum direction dir,
    map_t *map
)
{
    start_dest->x = orig_pos->x;
    start_dest->y = orig_pos->y;
    switch (dir) {
        case UP:
            start_dest->y = modulo(start_dest->y - 1, map->y);
            break;
        case RIGHT:
            start_dest->x = start_dest->x + 1;
            break;
        case DOWN:
            start_dest->y = start_dest->y + 1;
            break;
        case LEFT:
            start_dest->x = modulo(start_dest->x - 1, map->x);
            break;
        default:
            break;
    }
}

static bool get_min_distance(
    const pos_t *src_pos,
    const pos_t *tile_pos,
    double *old_dist,
    const pos_t *map_size
)
{
    pos_t vec_dist = {0};
    double new_dist = 0.f;

    vec_dist.x = abs(tile_pos->x - src_pos->x);
    vec_dist.y = abs(tile_pos->y - src_pos->y);
    if (vec_dist.x > map_size->x / 2)
        vec_dist.x = map_size->x - vec_dist.x;
    if (vec_dist.y > map_size->y / 2)
        vec_dist.y = map_size->y - vec_dist.y;
    new_dist = sqrt(vec_dist.x * vec_dist.x + vec_dist.y * vec_dist.y);
    if (new_dist < *old_dist) {
        *old_dist = new_dist;
        return true;
    }
    return false;
}

static int get_shortest_distance_sound(
    const pos_t *src_pos,
    pos_t *pos,
    enum direction dir,
    map_t *map
)
{
    double dist = DBL_MAX;
    size_t nb_move = 1;
    size_t i = 1;
    size_t sound_idx = 0;
    pos_t map_size = {map->x, map->y};

    dir = modulo((dir - 1), NB_DIR);
    for (; i <= 8; i++) {
        if (nb_move == 0) {
            dir = modulo((dir - 1), NB_DIR);
            nb_move = 2;
        }
        if (get_min_distance(src_pos, pos, &dist, &map_size))
            sound_idx = i;
        move_by_dir(pos, dir, map);
        nb_move--;
    }
    return sound_idx;
}

static void prepare_info_client(
    char const *msg,
    client_t *sender,
    client_t *cli,
    game_t *g
)
{
    enum direction dir = UP;
    pos_t pos = {0};
    int shortest_dir = 0;

    dir = cli->ai->dir;
    if (!is_coord_equal(&sender->ai->pos, &cli->ai->pos)) {
        get_starting_pos(&pos, &cli->ai->pos, dir, g->map);
        shortest_dir =
            get_shortest_distance_sound(&sender->ai->pos, &pos, dir, g->map);
    }
    prepare_response_cat(&cli->io, "message %d, %s\n", shortest_dir, msg);
}

static void send_to_everyone(
    char const *msg,
    struct client_list *clis,
    client_t *c,
    game_t *g
)
{
    for (size_t i = 0; i < clis->size; i++) {
        if (clis->data[i]->type == AI && valid_client(clis->data[i], c)) {
            prepare_info_client(msg, c, clis->data[i], g);
        }
    }
}

void handle_broadcast(client_t *cli, command_state_t *s)
{
    str_t *join_args = join_n_start_strs(s->args, " ", 1);
    char *msg = str_cstr(join_args);

    prepare_response_cat(&cli->io, "ok\n");
    send_to_everyone(msg, s->clients, cli, s->game);
    broadcast_to(GUI, s->clients, "pbc %d %s\n", cli->ai->id, msg);
    free(msg);
    str_free(join_args);
}
