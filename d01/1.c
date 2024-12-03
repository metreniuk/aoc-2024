#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void  printArr(int* arr, int size) {
    for (int i = 0; i < size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

void swap(int* a, int* b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

int partition(int* arr, int low, int high) {
    int pivot = arr[high];
    int i = low - 1;

    for (int j = low; j < high; j++) {
        if (arr[j] < pivot) {
            i++;
            swap(&arr[i], &arr[j]);
        }
    }

    swap(&arr[i + 1], &arr[high]);
    return i + 1;
}

void quicksort(int* arr, int low, int high) {
    if (low < high) {
        int pivot = partition(arr, low, high);
        quicksort(arr, low, pivot - 1);
        quicksort(arr, pivot + 1, high);
    }
}



int main(void) {
    FILE *fp = fopen("./input-0.txt", "r");
    size_t len = 0;
    char* line = NULL;

    int left[100] = {};
    int right[100] = {};
    int i = 0;
    while (getline(&line, &len, fp) != -1) {
        char* l = strtok(line, "   ");
        char* r = strtok(NULL, "   ");
        int li = atoi(l);
        int ri = atoi(r);

        left[i] = li;
        right[i] = ri;

        i++;

//         printf("%d %d\n", li, ri);
    }
    quicksort(left, 0, i);
    printArr(left, i);

    printArr(right, i);

    fclose(fp);

}