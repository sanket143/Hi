#include <stdio.h>

namespace compiler {
    bool numComparator(double left, int comparator, double right){
        bool bval = false;

        switch (comparator){
        case 1:
            bval = (left < right);
            break;
        case 2:
            bval = (left > right);
            break;
        case 3:
            bval = (left == right);
            break;
        case 4:
            bval = (left <= right);
            break;
        case 5:
            bval = (left >= right);
            break;
        case 6:
            bval = (left != right);
            break;
        case 7:
            bval = (left == right);
            break;
        default:
            fprintf(stderr, "%s", "Invalid comparator expression");
            break;
        }

        return bval;
    }
}