<?php

declare(strict_types=1);

namespace App\Queries;

use App\Models\Code;
use Illuminate\Database\Eloquent\ModelNotFoundException;

// The store and the exception struct live in other modules than the calls that throw.
function codeNamed(string $name): Code
{
    return Code::query()->where('name', $name)->sole();
}

function nameOf(int $id): string
{
    try {
        $code = Code::findOrFail($id);
        return $code->name;
    } catch (ModelNotFoundException $e) {
        return $e->getMessage();
    }
}
