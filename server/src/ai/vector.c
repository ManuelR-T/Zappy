/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** vector_ai
*/

#include <string.h>

#include "types/ai.h"
#include "types/game.h"
#include "vector.h"

bool resize_vector_ai(vector_ai_t *v)
{
    v->size *= 2;
    v->ais = realloc(v->ais, v->size * sizeof(ai_t));
    return false;
}

void push_back_vector_ai(vector_ai_t *v, const ai_t *nw)
{
    memcpy(v->ais + v->len, nw, sizeof(ai_t));
    v->len += 1;
}

vector_ai_t *init_ais(size_t ais_count)
{
    vector_ai_t *nw = calloc(1, sizeof(vector_ai_t));

    nw->ais = calloc(1, sizeof(ai_t));
    nw->len = 0;
    nw->size = ais_count;
    return nw;
}
