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
