<?php

declare(strict_types=1);

enum PaymentProcessingStatus: string
{
    case AwaitingCustomerConfirmation = 'AWAITING_CUSTOMER_CONFIRMATION';
    case Active = 'active';
    case InReview = 'review';
    case Archived = 'archived';
}

foreach (PaymentProcessingStatus::cases() as $status) {
    echo $status->value, "\n";
}
echo PaymentProcessingStatus::from('review')->name, "\n";
