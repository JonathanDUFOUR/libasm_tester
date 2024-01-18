######################################
#              COMMANDS              #
######################################
CARGO = ${shell which cargo}

#######################################
#              FUNCTIONS              #
#######################################
MANDATORY_FUNCTIONS = \
	${addprefix ft_, \
		strcmp \
		strcpy \
		strdup \
		strlen \
		read \
		write \
	}
BONUS_FUNCTIONS = \
	${addprefix ft_, \
		atoi_base \
		list_push_front \
		list_size \
		list_sort \
	}

#######################################
#             DIRECTORIES             #
#######################################
ASM_DIR = ..

#######################################
#              LIBRARIES              #
#######################################
        ASM = libasm
      ASM_A = ${ASM_DIR}/${ASM}.a
ASM_BONUS_A = ${ASM_DIR}/${ASM}_bonus.a

#######################################
#                FLAGS                #
#######################################
TEST_FLAGS = --no-fail-fast

#######################################
#                RULES                #
#######################################
.PHONY: mandatory bonus all ${MANDATORY_FUNCTIONS} ${BONUS_FUNCTIONS} clean fclean re fre

mandatory: ${ASM_A}
	${CARGO} test ${TEST_FLAGS} ${addprefix --test ,${MANDATORY_FUNCTIONS}} || true

bonus: ${ASM_BONUS_A}
	${CARGO} test ${TEST_FLAGS} ${addprefix --test ,${BONUS_FUNCTIONS}} || true

all: mandatory bonus

${MANDATORY_FUNCTIONS}: ${ASM_A}
	${CARGO} test ${TEST_FLAGS} --test $@ || true

${BONUS_FUNCTIONS}: ${ASM_BONUS_A}
	${CARGO} test ${TEST_FLAGS} --test $@ || true

${ASM_A} ${ASM_BONUS_A}:
	${MAKE} ${@F} -C ${@D}

clean:
	${CARGO} clean

fclean: clean
	${MAKE} fclean -C ${ASM_DIR}

re: clean all

fre: fclean all