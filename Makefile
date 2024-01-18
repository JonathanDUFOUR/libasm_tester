######################################
#              COMMANDS              #
######################################
CARGO = ${shell which cargo}

#######################################
#              FUNCTIONS              #
#######################################
MANDATORY_FUNCTIONS = \
	ft_strlen \
	ft_strcpy \
	ft_strcmp \
	ft_write \
	ft_read	\
	ft_strdup
BONUS_FUNCTIONS = \
	ft_atoi_base \
	ft_list_push_front \
	ft_list_size \
	ft_list_sort

#######################################
#             DIRECTORIES             #
#######################################
LIBASM_DIR = ..

#######################################
#                FLAGS                #
#######################################
TEST_FLAGS = --no-fail-fast

#######################################
#                RULES                #
#######################################
.PHONY: mandatory bonus all clean fclean re fre

mandatory:
	${MAKE} libasm.a -C ${LIBASM_DIR}
	${CARGO} test ${TEST_FLAGS} ${addprefix --test=, ${MANDATORY_FUNCTIONS}} || true

bonus:
	${MAKE} bonus -C ${LIBASM_DIR}
	${CARGO} test ${TEST_FLAGS} ${addprefix --test=, ${BONUS_FUNCTIONS}} || true

all: mandatory bonus

${MANDATORY_FUNCTIONS}:
	${MAKE} libasm.a -C ${LIBASM_DIR}
	${CARGO} test --test $@

${BONUS_FUNCTIONS}:
	${MAKE} bonus -C ${LIBASM_DIR}
	${CARGO} test --test $@

clean:
	${CARGO} clean

fclean: clean
	${MAKE} fclean -C ${LIBASM_DIR}

re: clean all

fre: fclean all