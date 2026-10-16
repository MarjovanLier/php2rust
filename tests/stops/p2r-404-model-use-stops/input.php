<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Ticket extends Model
{
}

// opened_at is a date column, left out of the struct.
function opened(Ticket $ticket): string
{
    return (string) $ticket->opened_at;
}

// This section reads models only.
function retitle(Ticket $ticket): void
{
    $ticket->title = 'new';
}

function addHit(Ticket $ticket): void
{
    $ticket->hits += 1;
}

function bump(Ticket $ticket): void
{
    $ticket->hits++;
}

function make(): Ticket
{
    return new Ticket();
}

function store(Ticket $ticket): bool
{
    return $ticket->save();
}

function all(): void
{
    Ticket::query();
}
