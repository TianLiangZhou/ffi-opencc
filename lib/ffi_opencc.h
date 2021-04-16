#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct BoxOpenCC BoxOpenCC;

struct BoxOpenCC *with_dict_path(const char *str);

char *s2t(const char *str, struct BoxOpenCC *oc);

char *t2s(const char *str, struct BoxOpenCC *oc);

char *s2tw(const char *str, struct BoxOpenCC *oc);

char *tw2s(const char *str, struct BoxOpenCC *oc);

char *s2hk(const char *str, struct BoxOpenCC *oc);

char *hk2s(const char *str, struct BoxOpenCC *oc);

void free_opencc(struct BoxOpenCC *oc);

void free_pointer(char *ptr);
