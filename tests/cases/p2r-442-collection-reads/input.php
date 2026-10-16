<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;

final class Code extends Model
{
    protected $fillable = ['name', 'kind', 'rank'];

    public $timestamps = false;
}

// The reads of a collection: a foreach without a key, count() in both forms, isEmpty(),
// isNotEmpty() and first(), on a parameter, a local variable and a call result.
/**
 * @param Collection<int, Code> $codes
 */
function summary(Collection $codes): string
{
    $text = $codes->isEmpty() ? 'empty' : 'some';
    if ($codes->isNotEmpty()) {
        $text .= ' ' . $codes->count() . '=' . count($codes);
    }
    $first = $codes->first();
    if ($first !== null) {
        $text .= ' first ' . $first->name;
    }
    foreach ($codes as $code) {
        $text .= ' ' . $code->id . $code->kind;
        $rank = $code->rank;
        if ($rank !== null) {
            $text .= '#' . $rank;
        }
    }
    return $text;
}

function firstName(string $kind): string
{
    $codes = Code::query()->where('kind', $kind)->get();
    $first = $codes->first();
    return $first === null ? '-' : $first->name;
}

function total(): int
{
    $all = Code::all();
    $copy = $all;
    return $copy->count() + count(Code::query()->where('kind', 'x')->get());
}

echo summary(Code::all()), "\n";
Code::create(['name' => 'a', 'kind' => 'x', 'rank' => 1]);
Code::create(['name' => 'b', 'kind' => 'y', 'rank' => null]);
Code::create(['name' => 'c', 'kind' => 'x', 'rank' => 3]);
echo summary(Code::all()), "\n";
echo summary(Code::query()->where('kind', 'x')->get()), "\n";
echo firstName('y'), ' ', firstName('z'), ' ', total(), "\n";
$any = Code::all()->first();
echo $any === null ? 'none' : $any->name, "\n";
