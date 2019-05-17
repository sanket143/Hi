#include <iostream>
#include <sstream>
#include <string>
#include <map>

#include "variable.hpp"

namespace compiler {
    // Var Setters
    void Var::setInt(int _int){
        v_int = _int;
        datatype = typeInt;
    }

    void Var::setDouble(double _double){
        v_double = _double;
        datatype = typeDouble;
    }

    void Var::setBool(bool _bool){
        v_bool = _bool;
        datatype = typeBool;
    }

    void Var::setString(std::string _string){
        v_string = _string;
        datatype = typeString;
    }

    void Var::print(){
        if(datatype == typeInt){
            std::cout << v_int;
        }
        else if(datatype == typeDouble){
            std::cout << v_double;
        }
        else if(datatype == typeBool){
            if(v_bool){
                std::cout << "true";
            } else {
                std::cout << "false";
            }
        }
        else if(datatype == typeString){
            std::cout << v_string;
        } else {
            std::cout << "Error: Unknown Data type" << std::endl;
        }
    }

    std::map<std::string, Var> variable_list;
    std::map<std::string, Var>::iterator variable_list_itr;

    void addNumVar(char *_varname, double _value){
        std::string temp(_varname);
        int intVal = (int) _value;

        variable_list_itr = variable_list.find(temp);

        if(variable_list_itr == variable_list.end()){
            if(intVal - _value){
                Var var(_value);
                variable_list.insert(std::pair<std::string, Var>(temp, var));
            } else {
                Var var(intVal);
                variable_list.insert(std::pair<std::string, Var>(temp, var));
            }
        } else {
            if(intVal - _value){
                variable_list_itr->second.setDouble(_value);
            } else {
                variable_list_itr->second.setInt(intVal);
            }
        }
    }

    void addStringVar(char *_varname, std::string _value){
        std::string temp(_varname);
        std::string temp_value(_value);

        variable_list_itr = variable_list.find(temp);

        if(variable_list_itr == variable_list.end()){
            Var var(temp_value);
            variable_list.insert(std::pair<std::string, Var>(temp, var));
        } else {
            variable_list_itr->second.setString(_value);
        }
    }

    double getNumValue(char *_varname){
        std::string temp(_varname);
        variable_list_itr = variable_list.find(temp);

        if(variable_list_itr != variable_list.end()){
            if(variable_list_itr->second.getType() == typeInt){
                return (double) variable_list_itr->second.getInt();
            }
            else if(variable_list_itr->second.getType() == typeString){
                return variable_list_itr->second.getString().length();
            }
            else if(variable_list_itr->second.getType() == typeBool){
                if(variable_list_itr->second.getBool()){
                    return 1;
                }

                return 0;
            }
            else if(variable_list_itr->second.getType() == typeDouble){
                return variable_list_itr->second.getDouble();
            }
        } else {
            return 0;
        }
    }

    char* getStringValue(char *_varname){
        std::string temp(_varname);
        variable_list_itr = variable_list.find(temp);

        if(variable_list_itr != variable_list.end()){
            if(variable_list_itr->second.getType() == typeString){
                temp = (std::string)variable_list_itr->second.getString();

            } else {
                std::ostringstream ss;

                if(variable_list_itr->second.getType() == typeInt){
                    int x = variable_list_itr->second.getInt();
                    ss << x;
                }
                else if(variable_list_itr->second.getType() == typeDouble){
                    double x = variable_list_itr->second.getDouble();
                    ss << x;
                }
                else if(variable_list_itr->second.getType() == typeBool){
                    bool x = variable_list_itr->second.getBool();
                    ss << x;
                }

                temp = ss.str();
            }

            return &temp[0u];
        } else {
            return "";
        }
    }
}