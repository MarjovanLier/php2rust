<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Database\MultipleRecordsFoundException;

final class Code extends Model
{
    protected $fillable = ['name', 'kind'];

    public $timestamps = false;
}

// Each form throws, so each function that calls one throws too.
function byName(string $name): string
{
    $code = Code::query()->where('name', $name)->sole();
    return $code->name . ' ' . $code->kind;
}

function firstOfKind(string $kind): string
{
    $code = Code::query()->where('kind', $kind)->firstOrFail();
    return $code->name;
}

function described(int $id): string
{
    $code = Code::findOrFail($id);
    $again = Code::query()->findOrFail($id);
    return $code->name . '/' . $again->kind;
}

function attempt(string $name): string
{
    try {
        return byName($name);
    } catch (ModelNotFoundException $e) {
        return 'missing: ' . $e->getMessage();
    } catch (MultipleRecordsFoundException $e) {
        return 'doubled: ' . $e->getMessage() . ' ' . $e->getCode();
    }
}

function findOrSay(int $id): string
{
    try {
        $code = Code::findOrFail($id);
        return $code->name;
    } catch (ModelNotFoundException $e) {
        return $e->getMessage();
    }
}

Code::create(['name' => 'a', 'kind' => 'x']);
Code::create(['name' => 'b', 'kind' => 'y']);
Code::create(['name' => 'b', 'kind' => 'z']);
// In the main code a form that throws stops the run when it throws.
$first = Code::query()->where('kind', 'z')->sole();
echo $first->id, ' ', byName('a'), "\n";
echo attempt('none'), "\n";
echo attempt('b'), "\n";
echo firstOfKind('y'), ' ', described(2), "\n";
echo findOrSay(3), ' ', findOrSay(9), "\n";
