#include <stdint.h>
typedef struct {
    int32_t context_id;
} CddContext;

CddContext cdd_c_init() {
    CddContext ctx;
    ctx.context_id = 1;
    return ctx;
}
