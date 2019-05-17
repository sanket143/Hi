#include <iostream>
#include <string>

#include "print.hpp"

namespace compiler {
    void print(char *var){
        std::string temp(var);
        variable_list_itr = variable_list.find(temp);
        if(variable_list_itr != variable_list.end()){
            (variable_list_itr->second).print();
        } else {
            std::cout << "Undefined variable '" << temp << "'\n";
        }
    }

    void printd(double d){
        int i = (int) d;
        if(i - d){
            std::cout << d;
        } else {
            std::cout << i;
        }
    }

    void printdn(double d){
        printd(d);
        std::cout << "\n";
    }

    void println(char *var){
        print(var);
        std::cout << "\n";
    }
}