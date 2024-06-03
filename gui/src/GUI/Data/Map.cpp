/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** Mapp
*/

#include "Map.hpp"
#include <raylib.h>
#include "Pos.hpp"
#include "Tile.hpp"

namespace GUI {
namespace Data {

Map::Map(int x, int y) : m_size({x, y})
{
    m_map.resize(x * y);
}

Map::Map(const Pos<int, 2> &pos) : m_size(pos)
{
    m_map.resize(pos.x() * pos.y());
}

Tile &Map::getTile(const Pos<int, 2> &pos)
{
    if (pos.x() >= m_size.x() || pos.y() >= m_size.y()) {
        throw std::out_of_range("Position out of map bounds");
    }
    return m_map.at(pos.y() * m_size.x() + pos.x());
}

Tile &Map::getTile(int x, int y)
{
    if (x >= m_size.x() || y >= m_size.y()) {
        throw std::out_of_range("Position out of map bounds");
    }
    return m_map.at(y * m_size.x() + x);
}

Tile Map::getTile(const Pos<int, 2> &pos) const
{
    if (pos.x() >= m_size.x() || pos.y() >= m_size.y()) {
        throw std::out_of_range("Position out of map bounds");
    }
    return m_map.at(pos.y() * m_size.x() + pos.x());
}

Tile Map::getTile(int x, int y) const
{
    if (x >= m_size.x() || y >= m_size.y()) {
        throw std::out_of_range("Position out of map bounds");
    }
    return m_map.at(y * m_size.x() + x);
}

std::vector<Player> &Map::getPlayers()
{
    return m_players;
}

std::vector<Egg> &Map::getEggs()
{
    return m_eggs;
}

Pos<int, 2> Map::getSize() const
{
    return m_size;
}

void Map::resize(int x, int y)
{
    m_map.resize(x * y);
    m_size = {x, y};
}

void Map::resize(const Pos<int, 2> &size)
{
    m_map.resize(size.x() * size.y());
    m_size = size;
}

void Map::displayTacticalView(int start_x, int start_y, int end_x, int end_y) const
{
    int mapWidth = end_x - start_x;
    int mapHeight = end_y - start_y;
    float tileSize = std::min(mapWidth / m_size.x(), mapHeight / m_size.y());

    for (int y = 0; y < m_size.y(); y++) {
        for (int x = 0; x < m_size.x(); x++) {
            auto &ressources = getTile(x, y).getRessources();

            float tileX = x * tileSize + start_x;
            float tileY = y * tileSize + start_y;

            for (int i = 0; i < 7; i++) {
                float ressourceX = tileX + (i % 3) * tileSize / 3;
                float ressourceY = tileY + (i / 3) * tileSize / 3;

                if (ressources[i] > 0 && ressources[i] < 1) {
                    DrawRectangle(ressourceX, ressourceY, tileSize / 3, tileSize / 3, ORANGE);
                } else if (ressources[i] >= 2) {
                    DrawRectangle(ressourceX, ressourceY, tileSize / 3, tileSize / 3, GREEN);
                } else {
                    DrawRectangle(ressourceX, ressourceY, tileSize / 3, tileSize / 3, RED);
                }
            }

            if (CheckCollisionPointRec(GetMousePosition(), {tileX, tileY, tileSize, tileSize})) {
                DrawRectangle(tileX, tileY, tileSize, tileSize, {0, 0, 0, 100});
            }
            DrawRectangleLines(tileX, tileY, tileSize, tileSize, BLACK);
        }
    }

    for (const auto &player : m_players) {
        int playerX = player.getPos().x() * tileSize + start_x;
        int playerY = player.getPos().y() * tileSize + start_y;

        DrawCircle(playerX + tileSize / 2, playerY + tileSize / 2, tileSize / 6, Color{0, 121, 241, 150});
    }

    for (const auto &egg : m_eggs) {
        int eggX = egg.getPosition().x() * tileSize + start_x;
        int eggY = egg.getPosition().y() * tileSize + start_y;

        DrawCircle(eggX + tileSize / 2, eggY + tileSize / 2, tileSize / 8, Color{253, 249, 0, 150});
    }
}

} // namespace Data
} // namespace GUI