initSidebarItems({"enum":[["CreateDestinationError","struct for typed errors of method [`create_destination`]"],["CreateSubscriptionError","struct for typed errors of method [`create_subscription`]"],["DeleteDestinationError","struct for typed errors of method [`delete_destination`]"],["DeleteSubscriptionByIdError","struct for typed errors of method [`delete_subscription_by_id`]"],["GetDestinationError","struct for typed errors of method [`get_destination`]"],["GetDestinationsError","struct for typed errors of method [`get_destinations`]"],["GetSubscriptionByIdError","struct for typed errors of method [`get_subscription_by_id`]"],["GetSubscriptionError","struct for typed errors of method [`get_subscription`]"]],"fn":[["create_destination","Creates a destination resource to receive notifications. The createDestination API is grantless. For more information, see Grantless operations in the Selling Partner API Developer Guide.  Usage Plan:  | Rate (requests per second) | Burst | | –– | –– | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see Usage Plans and Rate Limits in the Selling Partner API."],["create_subscription","Creates a subscription for the specified notification type to be delivered to the specified destination. Before you can subscribe, you must first create the destination by calling the createDestination operation.  Usage Plan:  | Rate (requests per second) | Burst | | –– | –– | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see Usage Plans and Rate Limits in the Selling Partner API."],["delete_destination","Deletes the destination that you specify. The deleteDestination API is grantless. For more information, see Grantless operations in the Selling Partner API Developer Guide.  Usage Plan:  | Rate (requests per second) | Burst | | –– | –– | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see Usage Plans and Rate Limits in the Selling Partner API."],["delete_subscription_by_id","Deletes the subscription indicated by the subscription identifier and notification type that you specify. The subscription identifier can be for any subscription associated with your application. After you successfully call this operation, notifications will stop being sent for the associated subscription. The deleteSubscriptionById API is grantless. For more information, see Grantless operations in the Selling Partner API Developer Guide.  Usage Plan:  | Rate (requests per second) | Burst | | –– | –– | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see Usage Plans and Rate Limits in the Selling Partner API."],["get_destination","Returns information about the destination that you specify. The getDestination API is grantless. For more information, see Grantless operations in the Selling Partner API Developer Guide.  Usage Plan:  | Rate (requests per second) | Burst | | –– | –– | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see Usage Plans and Rate Limits in the Selling Partner API."],["get_destinations","Returns information about all destinations. The getDestinations API is grantless. For more information, see Grantless operations in the Selling Partner API Developer Guide.  Usage Plan:  | Rate (requests per second) | Burst | | –– | –– | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see Usage Plans and Rate Limits in the Selling Partner API."],["get_subscription","Returns information about subscriptions of the specified notification type. You can use this API to get subscription information when you do not have a subscription identifier.  Usage Plan:  | Rate (requests per second) | Burst | | –– | –– | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see Usage Plans and Rate Limits in the Selling Partner API."],["get_subscription_by_id","Returns information about a subscription for the specified notification type. The getSubscriptionById API is grantless. For more information, see Grantless operations in the Selling Partner API Developer Guide.  Usage Plan:  | Rate (requests per second) | Burst | | –– | –– | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see Usage Plans and Rate Limits in the Selling Partner API."]]});