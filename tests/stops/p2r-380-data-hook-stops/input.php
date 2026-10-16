<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;

// laravel-data calls a hook as a public static method.
final class Hidden extends Data
{
    protected static function rules(): array
    {
        return [];
    }
}

final class Instance extends Data
{
    public function authorize(): bool
    {
        return true;
    }
}

// PHP checks #[\Override] against Data, which has no real rules().
final class Marked extends Data
{
    #[\Override]
    public static function rules(): array
    {
        return [];
    }
}

// PHP rejects two methods of one name in any letter case; Mago does not report it.
final class Twice extends Data
{
    public static function rules(): array
    {
        return [];
    }

    public static function RULES(): array
    {
        return [];
    }
}

interface Guarded
{
    public function authorize(): bool;
}

// The interface declares the name, so P2R-371 applies.
final class Implemented extends Data implements Guarded
{
    public static function authorize(): bool
    {
        return true;
    }
}

// A left-out hook is not a method of the class.
final class Calls extends Data
{
    public static function errorBag(): string
    {
        return 'calls';
    }

    public function check(): string
    {
        return self::errorBag() . static::errorBag() . Calls::errorBag() . $this->errorBag();
    }
}

// PHP rejects a bare return in a method with a return type when it compiles the file.
final class Bare extends Data
{
    public static function rules(): array
    {
        return;
    }
}

// PHP rejects these modifiers on a method when it compiles the file.
final class Modified extends Data
{
    public readonly static function rules(): array
    {
        return [];
    }

    private(set) static function messages(): array
    {
        return [];
    }

    public public static function redirect(): string
    {
        return 'home';
    }

    public static static function errorBag(): string
    {
        return 'bag';
    }

    final final static function attributes(): array
    {
        return [];
    }
}

// PHP rejects these parameters when it compiles the file, and Mago reports none of them.
final class Parameters extends Data
{
    public static function rules(int $this = 0): array
    {
        return [];
    }

    public static function messages(int|static $s = 0): array
    {
        return [];
    }

    public static function attributes(public string $x = 'a'): array
    {
        return [];
    }

    public static function redirect(int $a = 0, int $a = 1): string
    {
        return 'home';
    }

    public static function errorBag(int|INT $n = 0): string
    {
        return 'bag';
    }
}

// A hook's signature keeps to forms PHP always compiles; PHP rejects or deprecates these
// when it compiles the file, and Mago reports none of them.
final class Signatures extends Data
{
    public static function rules(?mixed $a = null): array
    {
        return [];
    }

    public static function messages(iterable|array $a = []): array
    {
        return [];
    }

    public static function attributes(string $a = PHP_EOL . 'x'): array
    {
        return [];
    }

    public static function redirect(int ...$a): string
    {
        return 'home';
    }

    public static function redirectRoute(int $a = null): string
    {
        return 'home';
    }

    public static function errorBag(): ?void
    {
    }

    // PHP deprecates a return by reference from a void function.
    public static function &authorize(): void
    {
    }
}

// Mago reports these under a code that a left-out hook drops.
final class Reserved extends Data
{
    public static function rules(): array
    {
        return [new \self()];
    }

    public static function messages(): array
    {
        function inner(): void
        {
        }

        return [];
    }

    public static function attributes(): array
    {
        return \parent::make();
    }

    public static function redirect(): string
    {
        return \static::make();
    }
}

// PHP removed, deprecates or warns about these when it compiles the file.
final class Deprecated extends Data
{
    public static function rules(): array
    {
        return [(integer) '1', (double) '2'];
    }

    public static function messages(): array
    {
        $x = 'a';
        return ["n${x}" => 'b'];
    }

    public static function attributes(): array
    {
        switch (1) {
            case 1;
                return [];
        }
        return [];
    }

    public static function redirect(): string
    {
        return `date`;
    }

    public static function errorBag(): string
    {
        return "\400";
    }
}

// PHP deprecates these too, in a closure and in a predefined variable.
final class AlsoDeprecated extends Data
{
    public static function rules(): array
    {
        $check = static fn (int $a = null): bool => $a === null;
        return [$check()];
    }

    public static function messages(): array
    {
        $join = static fn (string $a = 'x', string $b): string => $a . $b;
        return [$join('a', 'b')];
    }

    public static function attributes(): array
    {
        return [$http_response_header ?? []];
    }

    public static function redirect(): string
    {
        return "\u{110000}";
    }

    public static function errorBag(): string
    {
        declare(ticks=1);
        return 'bag';
    }

    public static function stopOnFirstFailure(?integer $a = null): bool
    {
        return true;
    }
}

// PHP rejects a class-like declared inside a method; an anonymous class brings members the
// tool does not check.
final class Nested extends Data
{
    public static function rules(): array
    {
        interface Helper
        {
        }

        return [];
    }

    public static function messages(): array
    {
        return [new class {
        }];
    }
}

// A finding on the docblock stops: it is not part of the hook.
final class Documented extends Data
{
    /** @return NoSuchClass */
    public static function redirect(): string
    {
        return 'home';
    }
}
