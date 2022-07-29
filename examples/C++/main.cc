#include <iostream>
#include <filesystem>
#include "ccsv.h"

int main() {
    CSV *csv = csv_new();
    csv_read(csv, std::filesystem::current_path().append("data/examples/input.csv").c_str());
    csv_print(csv);
    csv_write(csv, std::filesystem::current_path().append("data/examples/output.csv").c_str());

    csv_free(csv);

    return 0;
}
