/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** CheckBox
*/

#pragma once

#include "AButton.hpp"
#include <functional>

namespace GUI {

class CheckBox : public AButton<Raylib::Square, bool>
{
  public:
    CheckBox(const std::string &name, bool &val, MusicGame &music):
      AButton<Raylib::Square, bool>(name, val, music) {}

    void toDefault(void) override {
        this->m_state = DEFAULT;
    }

    void checkAction(const Raylib::Square &) override {
        if (Raylib::isMouseButtonDown(MOUSE_BUTTON_LEFT) || IsKeyDown(KEY_ENTER))
            this->m_state = PRESSED;
        else
            this->m_state = HOVER;
        if (Raylib::isMouseButtonReleased(MOUSE_BUTTON_LEFT) || IsKeyReleased(KEY_ENTER)) {
            this->m_val = !this->m_val;
            this->m_music.playSound();
        }
    }

    bool checkRecAction(const Raylib::Square &sqr) override {
        if (Raylib::checkCollisionMouseSquare(sqr)) {
            checkAction(sqr);
            return true;
        }
        this->m_state = DEFAULT;
        return false;
    }

    void draw(const Raylib::Square &sqr, int fontSize) override {
        Color colorRec = BLUE;
        Color colorBackRec = WHITE;
        Color colorText = BLACK;
        Raylib::Square lightRec = {sqr.x + 5, sqr.y + 5, sqr.size - 10};

        switch (this->m_state)
        {
        case HOVER:
            colorRec = GRAY;
            break;
        case PRESSED:
            colorRec = GREEN;
            break;
        default:
            break;
        }
        Raylib::drawSquare(sqr, colorBackRec);
        Raylib::drawSquareLines(sqr, colorRec);
        if (m_val)
            Raylib::drawSquare(lightRec, colorRec);
        Raylib::drawText(this->m_name, sqr.x + sqr.size + 20.0f, sqr.y + sqr.size / 4.0f, fontSize, colorText);
    }
};

} // namespace GUI
