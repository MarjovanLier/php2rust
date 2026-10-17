<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// PHP fails to parse an empty attribute group #[], which Mago accepts (P2R-500): on the
// casts() method of a model, which the output leaves out, and on a function.
final class Code extends Model
{
    public $timestamps = false;

    #[]
    protected function casts(): array
    {
        return ['code' => 'string'];
    }
}

#[]
function codeOf(Code $code): string
{
    return $code->code;
}

// Main converted this class, which PHP fails to parse.
final readonly class Point
{
    public function __construct(#[] public int $x)
    {
    }
}

// Only whitespace or a comment inside counts as empty too.
function twice(#[ ] int $value, #[/* none */] int $other): int
{
    return $value + $other;
}
