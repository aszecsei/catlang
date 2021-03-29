#define ARR_SIZE 50

int sumint(int *arr, int arrlen)
{
    int sum = 0;
    for (int i = 0; i < arrlen; ++i)
    {
        sum += arr[i];
    }
    return sum;
}

int main()
{
    int arr[ARR_SIZE];
    for (int i = 0; i < ARR_SIZE; ++i)
    {
        arr[i] = i + 1;
    }
    return sumint(arr, ARR_SIZE);
}