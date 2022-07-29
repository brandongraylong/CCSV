#include "ccsv.h"

int main() {
    CSV *csv = csv_new();
    csv_read(csv, "./data/examples/input.csv");
    csv_print(csv);
    csv_write(csv, "./data/examples/output.csv");

    csv_free(csv);

    return 0;
}
