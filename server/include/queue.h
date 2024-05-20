/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** queue
*/

#pragma once

#include <stdlib.h>
#include "command.h"

struct queue_node_s {
    command_t command;
    struct queue_node_s *next;
};

typedef struct {
    struct queue_node_s *node;
    size_t size_max;
    size_t len;
} queue_t;

enum queue_error_e {
    QUEUE_SUCCESS = 0,
    QUEUE_ERROR = 1,
    QUEUE_ALLOC_FAILED = 2
};

/**
 * @brief creates a new queue with a max size specified
 *
 * @param size_max the max size of the queue
 * @return the new queue
 */
queue_t *new_queue(size_t size_max);

/**
 * @brief free a queue
 *
 * @param q the queue to free
 */
void free_queue(queue_t *q);

/**
 * @brief push a command to the back of the queue
 *
 * @param q the queue
 * @param command the command to push
 * @return QUEUE_SUCESS or QUEUE_ERROR or QUEUE_ALLOC_FAILED
 */
enum queue_error_e queue_push_back(queue_t *q, command_t *command);
