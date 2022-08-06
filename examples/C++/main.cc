#include <iostream>
#include <filesystem>
#include <string>
#include "ccsv.h"

int main() {
    CSV* csv = csv_new();

    std::string input = std::filesystem::current_path().append("data/examples/output.csv").string();
    csv_read(csv, input.c_str());
    csv_print(csv);
    std::string output = std::filesystem::current_path().append("data/examples/output1.csv").string();
    csv_write(csv, output.c_str());
    
    const size_t* rows = csv_rows(csv);
    printf("Rows = ");
    if(rows == NULL) {
        printf("NULL");
    } else {
        printf("%ld", *rows);
    }
    printf("\n");

    const size_t* columns = csv_columns(csv);
    printf("Columns = ");
    if(columns == NULL) {
        printf("NULL");
    } else {
        printf("%ld", *columns);
    }
    printf("\n");

    csv_free(csv);

    return 0;
}
