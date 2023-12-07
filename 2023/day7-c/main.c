#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <assert.h>

#define MAX_HANDS 10000

unsigned hand_value(char *cards);

unsigned joker_hand_value(char *cards);

unsigned card_score(char c, unsigned joker_value);

unsigned cards_score(char *s, unsigned joker_value);

struct Hand {
    char *cards;
    int bid;
    unsigned value1;
    unsigned value2;
    unsigned score1;
    unsigned score2;
};

struct Input {
    struct Hand *hands[MAX_HANDS];
    size_t num_hands;
};

struct Hand *new_hand(char *cards, int bid) {
    char *s = strdup(cards);
    if (s == NULL) {
        fprintf(stderr, "cannot allocate card");
        abort();
    }
    struct Hand *hand = malloc(sizeof(struct Hand));
    if (hand == NULL) {
        fprintf(stderr, "cannot allocate hand");
        free(s);
        abort();
    }
    hand->cards = s;
    hand->bid = bid;
    hand->value1 = hand_value(cards);
    hand->value2 = joker_hand_value(cards);
    hand->score1 = cards_score(cards, 11);
    hand->score2 = cards_score(cards, 1);
    return hand;
}

struct Input *new_input() {
    struct Input *input = malloc(sizeof(struct Input));
    if (input == NULL) {
        fprintf(stderr, "cannot allocate input\n");
        abort();
    }
    input->num_hands = 0;
    return input;
}

void add_hand(struct Input *input, char *cards, int bid) {
    if (input->num_hands == MAX_HANDS) {
        fprintf(stderr, "More than %d hands. Maximum capacity exeeded\n", MAX_HANDS);
        abort();
    }
    struct Hand *hand = new_hand(cards, bid);
    input->hands[input->num_hands] = hand;
    input->num_hands++;
}


void free_hand(struct Hand *hand) {
    free(hand->cards);
    free(hand);
}

void free_input(struct Input *input) {
    size_t i;

    for (i = 0; i < input->num_hands; ++i) {
        free_hand(input->hands[i]);
    }

    free(input);
}


unsigned cards_score(char *s, unsigned joker_value) {
    char *p = s;
    unsigned n = 0;
    while (*p) {
        n *= 14;
        n += card_score(*p, joker_value);
        ++p;
    }
    return n;
}

unsigned card_score(char c, unsigned joker_value) {
    if (isdigit(c)) {
        return c - '0';
    }
    switch (c) {
        case 'A':
            return 14;
        case 'K':
            return 13;
        case 'Q':
            return 12;
        case 'J':
            return joker_value;
        case 'T':
            return 10;
    }
    return 0;
}

unsigned joker_hand_value(char *cards) {
    static char *card_values = "AKQJT987654321";
    char *p = card_values;
    char *q;
    char current_cards[10];
    unsigned n = 0;
    unsigned m = 0;

    while (*p) {
        strncpy(current_cards, cards, 5);
        current_cards[5] = 0;
        q = current_cards;
        while (*q) {
            if (*q == 'J') {
                *q = *p;
            }
            ++q;
        }
        ++p;
        m = hand_value(current_cards);
        if (m > n) {
            n = m;
        }
    }
    return n;
}

unsigned hand_value(char *cards) {
    unsigned card_counts[256];
    int i;
    unsigned hist[10];
    char *p = cards;

    memset(card_counts, 0, sizeof(card_counts));
    memset(hist, 0, sizeof(hist));

    while (*p) {
        ++card_counts[*p];
        ++p;
    }

    for (i = 0; i < 256; ++i) {
        assert(card_counts[i] <= 5);
        ++hist[card_counts[i]];
    }

    if (hist[5] > 0) {
        /* five of a kind */
        return 6;
    }

    if (hist[4]) {
        /* four of a kind */
        return 5;
    }

    if (hist[3] > 0 && hist[2] > 0) {
        /* full house */
        return 4;
    }

    if (hist[3] > 0) {
        /* three of a kind */
        return 3;
    }

    if (hist[2] == 2) {
        /* two pairs */
        return 2;
    }

    if (hist[2] == 1) {
        /* two pairs */
        return 1;
    }

    return 0;
}

int compare_hands1(struct Hand **h1, struct Hand **h2) {
    if ((*h1)->value1 < (*h2)->value1) {
        return -1;
    }
    if ((*h1)->value1 > (*h2)->value1) {
        return 1;
    }
    if ((*h1)->score1 < (*h2)->score1) {
        return -1;
    }
    if ((*h1)->score1 > (*h2)->score1) {
        return 1;
    }
    return 0;
}

int compare_hands2(struct Hand **h1, struct Hand **h2) {
    if ((*h1)->value2 < (*h2)->value2) {
        return -1;
    }
    if ((*h1)->value2 > (*h2)->value2) {
        return 1;
    }
    if ((*h1)->score2 < (*h2)->score2) {
        return -1;
    }
    if ((*h1)->score2 > (*h2)->score2) {
        return 1;
    }
    return 0;
}

int main() {
    char *filename = "day-7.txt";
    char *line = NULL;
    size_t len = 0;
    int read;
    char cards[10];
    int bid;
    int i;
    int n;

    struct Input *input = new_input();
    FILE *fp = fopen(filename, "r");
    if (fp == NULL) {
        perror(filename);
        free_input(input);
        return 1;
    }


    while ((read = getline(&line, &len, fp)) != -1) {
        if (sscanf(line, "%5s %d", cards, &bid) == 2) {
            add_hand(input, cards, bid);
        }
    }

    fclose(fp);
    if (line != NULL) {
        free(line);
    }

    qsort(
            input->hands,
            input->num_hands,
            sizeof(struct Hand *),
            (int (*)(const void *, const void *)) compare_hands1
    );
#if 0
    for (i = 0; i < input->num_hands; ++i) {
        printf("card: %s bid: %d value1: %d value2: %d score1: %d score2: %d\n",
               input->hands[i]->cards,
               input->hands[i]->bid,
               input->hands[i]->value1,
               input->hands[i]->value2,
               input->hands[i]->score1,
               input->hands[i]->score2
        );
    }
#endif
    n = 0;
    for (int i = 0; i < input->num_hands; ++i) {
        n += (i+1) * input->hands[i]->bid;
    }
    printf("%d\n", n);

    qsort(
            input->hands,
            input->num_hands,
            sizeof(struct Hand *),
            (int (*)(const void *, const void *)) compare_hands2
    );

#if 0
    for (i = 0; i < input->num_hands; ++i) {
        printf("card: %s bid: %d value1: %d value2: %d score1: %d score2: %d\n",
               input->hands[i]->cards,
               input->hands[i]->bid,
               input->hands[i]->value1,
               input->hands[i]->value2,
               input->hands[i]->score1,
               input->hands[i]->score2
        );
    }
#endif
    n = 0;
    for (int i = 0; i < input->num_hands; ++i) {
        n += (i+1) * input->hands[i]->bid;
    }
    printf("%d\n", n);

    free_input(input);
    return 0;
}