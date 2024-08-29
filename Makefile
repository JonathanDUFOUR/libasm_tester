######################################
#              COMMANDS              #
######################################
CARGO := ${shell which cargo}

#######################################
#              FUNCTIONS              #
#######################################
MANDATORY_FUNCTIONS := \
	${addprefix ft_, \
		strcmp \
		strcpy \
		strdup \
		strlen \
		read \
		write \
	}
OPTIONAL_FUNCTIONS := \
	${addprefix ft_, \
		memcpy \
	}
BONUS_FUNCTIONS := \
	${addprefix ft_, \
		atoi_base \
		${addprefix list_, \
			push_front \
			size \
			sort \
		} \
	}

#######################################
#             DIRECTORIES             #
#######################################
ASM_DIR := ..

#######################################
#              LIBRARIES              #
#######################################
        ASM := libasm
      ASM_A := ${ASM_DIR}/${ASM}.a
ASM_BONUS_A := ${ASM_DIR}/${ASM}_bonus.a

#######################################
#                FLAGS                #
#######################################
TEST_FLAGS := --no-fail-fast

#######################################
#                RULES                #
#######################################
.PHONY: mandatory
mandatory: ${ASM_A}
	${CARGO} test ${TEST_FLAGS} ${addprefix --test ,${MANDATORY_FUNCTIONS}} || true
ifeq (${BENCH}, 1)
	${CARGO} bench ${addprefix --bench ,${MANDATORY_FUNCTIONS}} || true
endif

.PHONY: optional
optional: ${ASM_A}
	${CARGO} test ${TEST_FLAGS} ${addprefix --test , ${OPTIONAL_FUNCTIONS}} || true
ifeq (${BENCH}, 1)
	${CARGO} bench ${addprefix --bench ,${OPTIONAL_FUNCTIONS}} || true
endif

.PHONY: bonus
bonus: ${ASM_BONUS_A}
	${CARGO} test ${TEST_FLAGS} ${addprefix --test ,${BONUS_FUNCTIONS}} || true
ifeq (${BENCH}, 1)
	${CARGO} bench ${addprefix --bench ,${BONUS_FUNCTIONS}} || true
endif

.PHONY: all
all: mandatory optional bonus

.PHONY: ${MANDATORY_FUNCTIONS} ${OPTIONAL_FUNCTIONS}
${MANDATORY_FUNCTIONS} ${OPTIONAL_FUNCTIONS}: ${ASM_A}
	${CARGO} test ${TEST_FLAGS} --test $@ || true
ifeq (${BENCH}, 1)
	${CARGO} bench --bench $@ || true
endif

.PHONY: ${BONUS_FUNCTIONS}
${BONUS_FUNCTIONS}: ${ASM_BONUS_A}
	${CARGO} test ${TEST_FLAGS} --test $@ || true
ifeq (${BENCH}, 1)
	${CARGO} bench --bench $@ || true
endif

.PHONY: ${ASM_A} ${ASM_BONUS_A}
${ASM_A} ${ASM_BONUS_A}:
	${MAKE} -C ${@D} ${@F}

.PHONY: clean
clean:
	${CARGO} clean

.PHONY: fclean
fclean: clean
	${MAKE} fclean -C ${ASM_DIR}

.PHONY: re
re: clean all

.PHONY: fre
fre: fclean all
