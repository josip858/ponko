#include "ponko.h"
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#define REQUIRE(x) do { if (!(x)) { fprintf(stderr, "failed: %s:%d: %s\n", __FILE__, __LINE__, #x); abort(); } } while (0)
int main(void) {
    PonkoWorld *w = ponko_world_new();
    uint32_t id = 0;
    REQUIRE(ponko_component_register(w, "Health", sizeof(float), _Alignof(float), &id) == CRABBY_OK);
    PonkoField f = {"hp", CRABBY_FIELD_F32, 0};
    REQUIRE(ponko_component_reflect(w, id, &f, 1) == CRABBY_OK);
    REQUIRE(ponko_resource_reflect(w, "Score", sizeof(float), _Alignof(float), &f, 1) == CRABBY_OK);
    const char *schema = "{\"Object\":[[\"hp\",\"Number\"]]}";
    REQUIRE(ponko_world_describe_component(w, "Health", schema) == CRABBY_OK);
    REQUIRE(ponko_world_describe_resource(w, "Score", schema) == CRABBY_OK);
    REQUIRE(ponko_world_describe_component(w, "Missing", schema) != CRABBY_OK);
    REQUIRE(ponko_world_describe_resource(w, "Missing", schema) != CRABBY_OK);
    REQUIRE(ponko_world_describe_component(w, "Health", "{") != CRABBY_OK);
    REQUIRE(ponko_world_describe_resource(w, "Score", "\"Unknown\"") != CRABBY_OK);
    REQUIRE(ponko_world_describe_component(NULL, "Health", schema) != CRABBY_OK);
    REQUIRE(ponko_world_describe_resource(NULL, "Score", schema) != CRABBY_OK);
    REQUIRE(ponko_world_describe_component(w, NULL, schema) != CRABBY_OK);
    REQUIRE(ponko_world_describe_resource(w, "Score", NULL) != CRABBY_OK);
    REQUIRE(ponko_world_describe_component(w, "Health", "\xff") != CRABBY_OK);
    REQUIRE(ponko_world_describe_resource(w, "\xff", schema) != CRABBY_OK);
    ponko_world_free(w);
}
