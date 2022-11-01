#!/usr/bin/env python3
from typing import List, Tuple #, Callable
# from json import loads
from enum import Enum
# from time import time


class TokenType(Enum):
    IDENTIFIER = ""
    WHITESPACE = ' '
    NUMBER = "0"
    DOUBLE = "0."
    DOT = '.'
    COLON = ':'
    COMMA = ','
    DASH = '-'
    BACKSLASH = '\\'
    OPEN_CURLY_BRACKET = '{'
    CLOSED_CURLY_BRACKET = '}'
    OPEN_SQUARE_BRACKET = '['
    CLOSED_SQUARE_BRACKET = ']'
    POSSIBLE_SPECIAL_CASE = "!"

    def is_of(self, instance: any) -> bool:
        return self is instance


class Token:

    value: str | int | bool | None

    def __init__(self, type: TokenType):
        self.type = type
        self.value = type.value

    def is_of(self, *instance: Tuple[any]) -> bool:
        return any(inst is self.type for inst in instance)

    def __repr__(self) -> str:
        # return f"{str(self.type).replace('TokenType.', '')}: '{self.value}'"
        return f"'{self.value}'"


class Lexer:

    __json: str
    __index: int
    __current_token: Token
    __return_queue: List[Token]
    __ignore_next_break: bool

    def __init__(self, json: str):
        self.__json = json
        self.__index = 0
        self.__current_token = Token(TokenType.WHITESPACE)
        self.__return_queue = []
        self.__ignore_next_break = False

    def tokenize(self) -> List[Token]:
        tokens: List[Token] = []
        while self.__has_next_token():
            token = self.__get_next_token()
            if not token.is_of(TokenType.WHITESPACE):
                tokens.append(token)
        return tokens

    def __has_next_token(self) -> bool:
        return self.__return_queue or self.__index < len(self.__json)

    def __get_next_token(self) -> Token:
        return_token = Token(TokenType.WHITESPACE)

        if self.__return_queue:
            return self.__return_queue.pop(0)

        for char in self.__json[self.__index:]:

            self.__index += 1

            if self.__ignore_next_break:
                self.__ignore_next_break = False
                self.__current_token.value += char
                continue

            match char:

                case ' ' | '\n' | '\r':
                    if self.__current_token.is_of(TokenType.IDENTIFIER):
                        self.__current_token.value += char
                    elif self.__current_token.is_of(TokenType.NUMBER, TokenType.DOUBLE):
                        return_token = self.__current_token
                        self.__end_token()
                        break
                    else:
                        return self.__current_token

                case '"':

                    if self.__current_token.is_of(TokenType.WHITESPACE):
                        self.__current_token = Token(TokenType.IDENTIFIER)
                    elif self.__current_token.is_of(TokenType.IDENTIFIER):
                        return_token = self.__current_token
                        self.__end_token()
                        break
                    else:
                        raise SyntaxWarning

                case '\\':
                    if self.__current_token.is_of(TokenType.IDENTIFIER):
                        self.__ignore_next_break = True
                    else:
                        raise SyntaxWarning

                case '-':
                    if self.__current_token.is_of(TokenType.IDENTIFIER):
                        self.__current_token.value += char
                    elif self.__current_token.is_of(TokenType.WHITESPACE):
                        self.__current_token = Token(TokenType.NUMBER)
                        self.__current_token.value = char
                    else:
                        raise SyntaxWarning

                case '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9':
                    if self.__current_token.is_of(TokenType.WHITESPACE):
                        self.__current_token = Token(TokenType.NUMBER)
                        self.__current_token.value = char
                    elif self.__current_token.is_of(TokenType.IDENTIFIER, TokenType.NUMBER, TokenType.DOUBLE):
                        self.__current_token.value += char
                    else:
                        raise SyntaxWarning

                case '.':
                    if self.__current_token.is_of(TokenType.IDENTIFIER, TokenType.DOUBLE):
                        self.__current_token.value += char
                    elif self.__current_token.is_of(TokenType.NUMBER):
                        val = self.__current_token.value
                        self.__current_token = Token(TokenType.DOUBLE)
                        self.__current_token.value = val + char
                    else:
                        raise SyntaxWarning

                case ':':
                    if self.__current_token.is_of(TokenType.IDENTIFIER):
                        self.__current_token.value += char
                    else:
                        return_token = Token(TokenType.COLON)
                        self.__end_token()
                        break

                case ',':
                    if self.__current_token.is_of(TokenType.IDENTIFIER):
                        self.__current_token.value += char
                    else:
                        return_token = self.__current_token
                        self.__end_token()
                        self.__return_queue.append(Token(TokenType.COMMA))
                        break

                case '{':
                    if self.__current_token.is_of(TokenType.IDENTIFIER):
                        self.__current_token.value += char
                    else:
                        return_token = Token(TokenType.OPEN_CURLY_BRACKET)
                        self.__end_token()
                        break

                case '}':
                    if self.__current_token.is_of(TokenType.IDENTIFIER):
                        self.__current_token.value += char
                    elif self.__current_token.is_of(TokenType.NUMBER, TokenType.DOUBLE):
                        return_token = self.__current_token
                        self.__end_token()
                        self.__return_queue.append(
                            Token(TokenType.CLOSED_CURLY_BRACKET))
                        break
                    else:
                        return_token = Token(TokenType.CLOSED_CURLY_BRACKET)
                        self.__end_token()
                        break

                case '[':
                    if self.__current_token.is_of(TokenType.IDENTIFIER):
                        self.__current_token.value += char
                    else:
                        return_token = Token(TokenType.OPEN_SQUARE_BRACKET)
                        self.__end_token()
                        break

                case ']':
                    if self.__current_token.is_of(TokenType.IDENTIFIER):
                        self.__current_token.value += char
                    elif self.__current_token.is_of(TokenType.NUMBER, TokenType.DOUBLE):
                        return_token = self.__current_token
                        self.__end_token()
                        self.__return_queue.append(
                            Token(TokenType.CLOSED_SQUARE_BRACKET))
                        break
                    else:
                        return_token = Token(TokenType.CLOSED_SQUARE_BRACKET)
                        self.__end_token()
                        break

                case _:
                    if self.__current_token.is_of(TokenType.POSSIBLE_SPECIAL_CASE):
                        if char not in {'n', 'u', 'l', 't', 'r', 'e', 'f', 'a', 's'}:
                            raise SyntaxWarning

                        self.__current_token.value += char

                        val = self.__current_token.value

                        if len(val) > 4 and val != "false":
                            raise SyntaxWarning

                        match val:
                            case "null":
                                self.__current_token.value = None
                                return_token = self.__current_token
                                self.__end_token()
                                break
                            case "false":
                                self.__current_token.value = False
                                return_token = self.__current_token
                                self.__end_token()
                                break
                            case "true":
                                self.__current_token.value = True
                                return_token = self.__current_token
                                self.__end_token()
                                break

                    elif not self.__current_token.is_of(TokenType.IDENTIFIER, TokenType.DOUBLE, TokenType.NUMBER):
                        if char == '\n':
                            continue
                        if char not in {'n', 'f', 't'}:
                            raise SyntaxWarning
                        self.__current_token = Token(
                            TokenType.POSSIBLE_SPECIAL_CASE
                        )
                        self.__current_token.value = char
                    else:
                        self.__current_token.value += char

        if return_token.is_of(TokenType.NUMBER):
            return_token.value = int(return_token.value)
        elif return_token.is_of(TokenType.DOUBLE):
            return_token.value = float(return_token.value)

        return return_token

    def __end_token(self):
        self.__current_token = Token(TokenType.WHITESPACE)


class Parser:

    __delimeters = {
        TokenType.OPEN_CURLY_BRACKET: TokenType.CLOSED_CURLY_BRACKET,
        TokenType.OPEN_SQUARE_BRACKET: TokenType.CLOSED_SQUARE_BRACKET
    }

    @staticmethod
    def parse(tokens: List[Token]) -> dict:
        delimeters = Parser.__delimeters
        stack: List[Token] = []

        toks: List[List[Token]] = []
        scope: List[Token] = None

        for token in tokens:
            if isinstance(token, dict) or isinstance(token, list):
                scope.append(token)
            elif token.type in delimeters:
                stack.append(token.type)
                if scope is not None:
                    toks.append(scope)
                scope = []
                scope.append(token)
            elif len(stack) > 0 and delimeters[stack[-1]] is token.type:
                stack.pop()
                scope.append(token)
                toks.append(scope)
                scope = []
            else:
                scope.append(token)

        if stack:
            raise SyntaxWarning

        if len(toks) == 1:
            tokens = toks[0]
            tokens.pop()

            result: dict | list = None

            start_delim = tokens.pop(0)
            if start_delim.is_of(TokenType.OPEN_CURLY_BRACKET):
                result = dict()
                next_expected: Token = TokenType.IDENTIFIER
                key = None
                for token in tokens:
                    if isinstance(token, dict) or isinstance(token, list):
                        result[key] = token
                        key = None
                        next_expected = TokenType.COMMA
                    elif next_expected is None:
                        result[key] = token.value
                        key = None
                        next_expected = TokenType.COMMA
                    elif not token.is_of(next_expected):
                        raise SyntaxWarning
                    elif next_expected.is_of(TokenType.COLON):
                        next_expected = None
                    elif next_expected.is_of(TokenType.COMMA):
                        next_expected = TokenType.IDENTIFIER
                    elif next_expected.is_of(TokenType.IDENTIFIER):
                        key = token.value
                        next_expected = TokenType.COLON
            elif start_delim.is_of(TokenType.OPEN_SQUARE_BRACKET):
                result = []
                next_expected: Token = None
                for token in tokens:
                    if isinstance(token, dict) or isinstance(token, list):
                        result.append(token)
                        next_expected = TokenType.COMMA
                    elif next_expected is None:
                        result.append(token.value)
                        next_expected = TokenType.COMMA
                    elif not token.is_of(next_expected):
                        raise SyntaxWarning
                    elif next_expected.is_of(TokenType.COMMA):
                        next_expected = None
            else:
                raise SyntaxWarning

            return result

        new_tokens: List[Token] = []
        for tokens in toks:
            if Parser.__is_parseable(tokens):
                new_tokens.append(Parser.parse(tokens))
            else:
                for tok in tokens:
                    new_tokens.append(tok)

        return Parser.parse(new_tokens)

    @staticmethod
    def __is_parseable(tokens: List[Token]) -> bool:

        delimeters = Parser.__delimeters

        stack: List[Token] = []

        has_popped_at_least_once = False

        for token in tokens:
            if isinstance(token, dict) or isinstance(token, list):
                continue
            elif token.type in delimeters:
                stack.append(token.type)
            elif len(stack) > 0 and delimeters[stack[-1]] is token.type:
                stack.pop()
                has_popped_at_least_once = True
            elif token.type in delimeters.values():
                return False

        return not stack and has_popped_at_least_once


# def time_func(f: Callable):
#     start = time()
#     res = f()
#     end = time()
#     print(end - start)
#     return res


def parse(json: str) -> dict:
    return Parser.parse(Lexer(json).tokenize())


# with open("a.json", 'r') as f:
#     json = ''.join(s.strip() for s in f.readlines())

# x = time_func(lambda: parse(json))
# y = time_func(lambda: loads(json))
# assert x == y
