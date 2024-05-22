/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** ai
*/

#pragma once

#include <stdlib.h>
#include "map.h"

typedef struct tile_content_s inventory_t;

typedef struct {
    char *team;
    size_t x;
    size_t y;
    size_t level;

    inventory_t inventory;
    long time_to_wait; // if current time < this time dont do anything
} ai_t;