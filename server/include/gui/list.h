/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** list
*/

#pragma once

#include "defs.h"

static const gui_cmd_t CMDS[] = {
    {"msz", &map_size},
    {"bct", &map_content_tile},
    {"mct", &map_content_full},
    {NULL, NULL},
};