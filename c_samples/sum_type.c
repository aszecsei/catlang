typedef struct
{
    void *obj;
    int typ;
} World;

int example(World w)
{
    if (w.typ == 63 /* Moon */)
    {
        return 1;
    }
    else
    {
        return 0;
    }
}