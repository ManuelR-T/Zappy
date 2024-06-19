/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** handle_fork
*/

#include "core/client.h"
#include "core/router/route.h"
#include "core/types/client.h"
#include "core/types/team.h"
#include "logger.h"

void handle_fork(client_t *cli, command_state_t *s)
{
    egg_t *egg = create_egg(s->game->map->x, s->game->map->y);

    if (!egg) {
        logs(ERROR_LEVEL, "Allocation error on fork handling");
        return;
    }
    logs(DEBUG, "Client %d AI %d is forking\n", cli->fd, cli->ai->id);
    egg->pos.x = cli->ai->pos.x;
    egg->pos.y = cli->ai->pos.y;
    queue_pushback_queue_egg_t(cli->ai->team->eggs, egg);
    prepare_response_cat(&cli->io, "ok\n");
    broadcast_to(GUI, s->clients, "pfk %d\n", cli->ai->id);
    broadcast_to(
        GUI,
        s->clients,
        "enw %d %d %d %d\n",
        egg->id,
        cli->ai->id,
        egg->pos.x,
        egg->pos.y
    );
    logs(
        INFO,
        "Team %s has a new egg [%d]\n",
        cli->ai->team->name,
        cli->ai->team->eggs->size
    );
}
