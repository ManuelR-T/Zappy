/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** eat
*/

#include "client.h"
#include "clock.h"
#include "server.h"
#include "types/ai.h"
#include "utils.h"

static void send_death(int n, server_t *s)
{
    for (__auto_type i = 0; i < SOMAXCONN; i++) {
        if (s->clients[i].fd > 0 && s->clients[i].type == GUI) {
            send_client(&s->clients[i], "edi %d\n", n);
        }
    }
}

void make_ai_eat(client_t *cli, server_t *server, int n)
{
    if (!cli->ai->alive || !has_n_ticks_passed(cli->ai->food_clock, 126)) {
        return;
    }
    if (cli->ai->inventory.food == 0) {
        logger_info("Ai %d is dead\n", n);
        send_client(cli, "dead\n");
        send_death(n, server);
    }
    logger_info("Cli %d is eating\n", n);
    cli->ai->inventory.food -= 1;
    reset_clock(cli->ai->food_clock);
    cli->ai->alive = false;
}