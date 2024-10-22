/*
** EPITECH PROJECT, 2024
** Zappy
** File description:
** ArgParser
*/

#include <algorithm>

#include "ArgParser.hpp"

namespace ArgParser {

bool ArgParser::has(const std::string &param) const
{
    return params_.find(param) != params_.end();
}

template <>
bool ArgParser::get<bool>(const std::string &param) const
{
    if (params_.find(param) == params_.end()) {
        throw Error("Parameter not found: " + param, "ArgParser::get");
    } else if (types_.at(param) != typeid(bool).name()) {
        throw Error("Type mismatch for parameter: " + param, "ArgParser::get");
    }
    std::string value = params_.at(param);
    std::transform(value.begin(), value.end(), value.begin(), ::tolower);
    if (value == "true" || value == "1") {
        return true;
    } else if (value == "false" || value == "0") {
        return false;
    } else {
        throw Error("Invalid value for bool: " + value, "ArgParser::get<bool>");
    }
}

template <>
std::string ArgParser::get<std::string>(const std::string &param) const
{
    if (params_.find(param) == params_.end()) {
        throw Error("Parameter not found: " + param, "ArgParser::get");
    }
    return params_.at(param);
}

void ArgParser::checkMandatoryArgs(const std::set<std::string> &providedArgs) const
{
    for (const auto &arg : mandatoryArgs_) {
        if (providedArgs.find(arg) == providedArgs.end()) {
            throw Error("Mandatory parameter missing: " + arg, "ArgParser::checkMandatoryArgs");
        }
    }
}

bool ArgParser::isValidType(const std::string &key, const std::string &value) const
{
    std::istringstream ss(value);
    if (types_.at(key) == typeid(int).name()) {
        int intValue;
        ss >> intValue;
        return !ss.fail() && ss.eof();
    } else if (types_.at(key) == typeid(bool).name()) {
        return (value == "true" || value == "false");
    } else if (types_.at(key) == typeid(std::string).name()) {
        return true;
    }
    return false;
}

void ArgParser::parse(int argc, char *argv[])
{
    std::set<std::string> providedArgs;

    for (int i = 1; i < argc; i++) {
        std::string param(argv[i]);

        if (param[0] == '-') {
            std::string key = param.substr(1);

            if (i + 1 < argc) {
                std::string value = argv[++i];

                if (types_.find(key) != types_.end()) {
                    if (!isValidType(key, value)) {
                        throw Error("Invalid value for parameter: " + key, "ArgParser::parseArgs");
                    }
                    params_[key] = value;
                    providedArgs.insert(key);
                } else {
                    throw Error("Unknown parameter: " + key, "ArgParser::parseArgs");
                }
            } else {
                throw Error("Missing value for parameter: " + key, "ArgParser::parseArgs");
            }
        } else {
            throw Error("Invalid parameter format: " + param, "ArgParser::parseArgs");
        }
    }

    checkMandatoryArgs(providedArgs);
}
}; // namespace ArgParser
