/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** Display
*/

#pragma once

#include <string>
#include <vector>

#include "../Network/Handler.hpp"
#include "Menu/Menu.hpp"
#include "Menu/Settings.hpp"
#include "Data/Map.hpp"
#include "MessageBox.hpp"
#include "TimeUnitInput.hpp"
#include "ServerMessageHandler.hpp"
#include "define.hpp"

namespace GUI {

class ServerMessageHandler;

class Display {
  public:
    Display(Network::Handler &networkHandler, bool debug, int width = 1920, int height = 1080);
    ~Display();
    void run();

    Data::Map &getMap()
    {
        return map;
    }

    void setTimeUnit(int time)
    {
        timeUnitInput.setTimeUnit(time);
    }

    void setEndGame(std::vector<std::string> message, bool end = true)
    {
        endGame = end;
        endGameMessage = message;
    }

    int getTimeUnit() const
    {
        return timeUnitInput.getTimeUnit();
    }

    void addMessage(std::string message, int user = SERVER)
    {
        messageBox.addMessage(message, user);
    }

    std::vector<std::string> team;

  private:
    void displayMenu();
    void displaySettings();
    void displayGame();
    void handleEvent();
    void handleServerMessage();
    void resize();

    Network::Handler &networkHandler;
    ServerMessageHandler serverMessageHandler;
    bool debug;
    Data::Map map;
    bool endGame;
    std::vector<std::string> endGameMessage;
    int screenWidth, screenHeight;
    int offsetX, offsetY, newWidth, newHeight;
    MessageBox messageBox;
    InfoBox infoBox;
    TimeUnitInput timeUnitInput;

    Camera3D m_cam;

    Menu m_menu;
    Settings m_settings;
};

} // namespace GUI
