<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;

// A hook body with a part outside the closed list of P2R-510 stops at that part. PHP
// rejects most of these when it compiles the file; main converted all but four, which
// Mago reports itself (continue outside a loop, $this as a foreach target or a read in a
// static method, and an empty array element).

// The forms that the review of slice 41 found, part 1.
final class Statics extends Data
{
    public function __construct(public string $number) {}

    public static function rules(): array
    {
        static $x = 1;
        static $x = 2;
        return [];
    }

    public static function messages(): array
    {
        goto missing;
        return [];
    }

    public static function attributes(): array
    {
        unset($this);
        return [];
    }

    public static function stopOnFirstFailure(): bool
    {
        return str_contains(haystack: 'a', 'b');
    }

    public static function redirect(): string
    {
        $a = [];
        [] = $a;
        return 'x';
    }

    public static function redirectRoute(): string
    {
        return 1 instanceof Statics ? 'a' : 'b';
    }

    public static function errorBag(): string
    {
        return match (1) {
            default => 'a',
            default => 'b',
        };
    }

    public static function authorize(): bool
    {
        $f = fn($a, $a) => true;
        return $f(1, 2);
    }
}

// Part 2.
final class Labels extends Data
{
    public function __construct(public string $number) {}

    public static function rules(): array
    {
        yield 1;
        return [];
    }

    public static function messages(): array
    {
        l:
        l:
        return [];
    }

    public static function attributes(): array
    {
        $x = <<<'EOT'
      a
        EOT;
        return [];
    }

    public static function stopOnFirstFailure(): bool
    {
        $f = fn($this) => true;
        return $f(1);
    }

    public static function redirect(): string
    {
        $o = null;
        $r = &$o?->x;
        return 'x';
    }

    public static function redirectRoute(): string
    {
        global $this;
        return 'x';
    }

    public static function errorBag(): string
    {
        $f = function () use ($_GET) {
            return 'x';
        };
        return $f();
    }

    public static function authorize(): bool
    {
        $o = null;
        unset($o?->x);
        return str_repeat(string: 'x', ...[2]) === 'xx';
    }
}

// Part 3: the pipe with an arrow function, and a variadic parameter before the last.
final class Pipes extends Data
{
    public function __construct(public string $number) {}

    public static function rules(): array
    {
        return [] |> fn(array $x): array => $x;
    }

    public static function authorize(): bool
    {
        $f = static fn(int ...$b, int $c): bool => true;
        return true;
    }
}

// A closure needs the return type void, no default, and continue only inside a loop.
final class Closures extends Data
{
    public function __construct(public string $number) {}

    public static function rules(): array
    {
        return ['a' => [function (string $a) {}]];
    }

    public static function messages(): array
    {
        return ['a' => [function (string $a = 'x'): void {}]];
    }

    public static function attributes(): array
    {
        return ['a' => [function (string $a): void {
            continue;
        }]];
    }
}

// No else, no parameter name twice, no arrow function.
final class Branches extends Data
{
    public function __construct(public string $number) {}

    public static function rules(): array
    {
        return ['a' => [function (string $a): void {
            if ($a === '') {
                return;
            } else {
                return;
            }
        }]];
    }

    public static function messages(): array
    {
        return ['a' => [function (string $a, string $a): void {}]];
    }

    public static function attributes(): array
    {
        return ['a' => [fn (string $a): bool => true]];
    }
}

// A foreach writes only plain variables, and a static hook has no $this.
final class Targets extends Data
{
    public function __construct(public string $number) {}

    public static function rules(): array
    {
        return ['a' => [function (array $v): void {
            foreach ($v as $this) {
                return;
            }
        }]];
    }

    public static function messages(): array
    {
        return ['a' => [function (array $v): void {
            foreach ($v as $GLOBALS) {
                return;
            }
        }]];
    }

    public static function attributes(): array
    {
        return ['a' => [$this]];
    }
}

// An array literal is [..] without ... or an empty element.
final class Arrays extends Data
{
    public function __construct(public string $number) {}

    public static function rules(): array
    {
        return [...['a' => []]];
    }

    public static function messages(): array
    {
        return ['a', , 'b'];
    }

    public static function attributes(): array
    {
        return array('a' => 'b');
    }
}

// A class is a plain name; parentheses are not on the list.
final class Names extends Data
{
    public function __construct(public string $number) {}

    public static function rules(): array
    {
        return ['a' => [parent::rules()]];
    }

    public static function messages(): array
    {
        return ['a' => [new static('x')]];
    }

    public static function attributes(): array
    {
        return ('a' === 'b') ? [] : [];
    }
}
