#pragma once

#include <iterator>
#include <iostream>
#include <string>
#include <map>

namespace compiler {
    enum typeTag {
        typeInt,
        typeString, 
        typeDouble,
        typeBool
    };

    class Var {
        int v_int;
        double v_double;
        bool v_bool;
        std::string v_string;
        typeTag datatype;

      public:
        Var(int _int): v_int(_int){
            datatype = typeInt;
        }

        Var(double _double): v_double(_double){
            datatype = typeDouble;
        }

        Var(std::string _string): v_string(_string){
            datatype = typeString;
        }

        Var(bool _bool): v_bool(_bool){
            datatype = typeBool;
        }

        // Getters
        int getInt(){
            return v_int;
        }

        double getDouble(){
            return v_double; 
        }

        bool getBool(){
            return v_bool;
        }

        std::string getString(){
            return v_string;
        }

		enum typeTag getType(){
			return datatype;
		}

        std::string type(){
            std::string _datatype;
            switch (datatype){
                case typeInt:
                    _datatype = "Integer";
                    break;
                case typeDouble:
                    _datatype = "Double";
                    break;
                case typeString:
                    _datatype = "String";
                    break;
                case typeBool:
                    _datatype = "Boolean";
                    break;
                default:
                    _datatype = "Unknown";
                    break;
            }

            return _datatype;
        }

        // Setters
        void setInt(int);
        void setDouble(double);
        void setBool(bool);
        void setString(std::string);

        void print();
    };

    extern std::map<std::string, Var> variable_list;
    extern std::map<std::string, Var>::iterator variable_list_itr;

    void addNumVar(char *, double);
    void addStringVar(char *, std::string);
    void addBoolVar(char *, bool);

    double getNumValue(char *);
    char* getStringValue(char *);
}