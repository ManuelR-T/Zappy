/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** Menu
*/

#include "Menu.hpp"


namespace GUI {

Menu::Menu(Network::Handler &networkHandler, Raylib::RecWin &newWindow):
    networkHandler(networkHandler), m_newWindow(newWindow), m_close(false), m_inGame(false), m_inSettings(false),
    m_iselected_but(0), modeKey(false)
{
    m_button.push_back(Button<Rectangle, bool>("start", m_inGame, [](bool &val){val = true;}));
    m_button.push_back(Button<Rectangle, bool>("settings", m_inSettings, [](bool &val){val = true;}));
    m_button.push_back(Button<Rectangle, bool>("quit", m_close, [](bool &val){val = true;}));
    m_iselected_but = m_button.size() - 1;
}

void Menu::display()
{
    int fontSize = HeightToFontSize(m_newWindow.height);
    Rectangle rec = {
        m_newWindow.x + m_newWindow.width * MulMenuPosButton,
        m_newWindow.y + m_newWindow.height * MulStrtY,
        m_newWindow.width * MulMenuButX,
        fontSize + SpacingY};
    std::string name = "ZAPPY";
    int sizeText = Raylib::getMeasureTextEx(name, fontSize).x;
    bool inGame = m_inGame;

    Raylib::drawText(name,
        m_newWindow.x + (m_newWindow.width - sizeText) / 2.0f,
        m_newWindow.y + m_newWindow.height * MulNameY,
        fontSize, PURPLE);
    for (size_t i = 0; i < m_button.size(); i++) {
        Button<Rectangle, bool> &but = m_button.at(i);

        if (!modeKey && but.checkRecAction(rec))
            m_iselected_but = i;
        if (modeKey) {
            if (m_iselected_but == i)
                but.checkAction();
            else
                but.toDefault();
        }
        but.draw(rec, fontSize * MulButFontSize);
        rec.y += rec.height;
    }
    if (inGame != m_inGame) {
        networkHandler.start();
    }
}

void Menu::eventhandler()
{
    Vector2 cursorMove = GetMouseDelta();
    if (modeKey && (cursorMove.x > 0.001f || cursorMove.y > 0.001f )) {
        modeKey = false;
        std::cout << "MODE CURSOR" << std::endl;
    }
    if (Raylib::isKeyPressed(KEY_UP)) {
        m_iselected_but = (m_iselected_but == 0) ? m_button.size() - 1 : m_iselected_but - 1;
        modeKey = true;
    }
    if (Raylib::isKeyPressed(KEY_DOWN)) {
        m_iselected_but = ((m_iselected_but + 1) % m_button.size());
        modeKey = true;
    }
}

void Menu::setClose(bool val)
{
    m_close = val;
}

void Menu::setInGame(bool val)
{
    m_inGame = val;
}

void Menu::setInSettings(bool val)
{
    m_inSettings = val;
}

bool Menu::getClose() const
{
    return m_close;
}

bool Menu::getInGame() const
{
    return m_inGame;
}

bool Menu::getInSettings() const
{
    return m_inSettings;
}

} // namespace GUI
