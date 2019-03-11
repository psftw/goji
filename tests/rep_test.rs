extern crate goji;
extern crate serde_json;

use goji::*;
use goji::rep::{Priority, Project, IssueType};

const JIRA_HOST: &str = "http://jira.com";

#[test]
fn issue_getters() {
    // Anonymized example from JIRA cloud
    let issue_str = r#"{
        "self": "https://jira.com/rest/agile/1.0/issue/1234",
        "id": "1234",
        "key": "MYPROJ-1234",
        "fields": {
            "assignee": {
                "self": "https://jira.com/rest/api/2/user?accountId=123456%3A00000000-0000-0000-0000-000000000000",
                "name": "example.user",
                "key": "example.user",
                "displayName": "Example User",
                "active": true,
                "timeZone": "America/New_York",
                "emailAddress": "example.user@example.com",
                "avatarUrls": {
                    "48x48": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=48&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D48%26noRedirect%3Dtrue",
                    "24x24": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=24&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D24%26noRedirect%3Dtrue",
                    "16x16": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=16&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D16%26noRedirect%3Dtrue",
                    "32x32": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=32&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D32%26noRedirect%3Dtrue"
                }
            },
            "attachment": [{
                "self": "https://jira.com/rest/api/2/attachment/45404",
                "id": "45404",
                "filename": "image-2018-12-10-09-12-14-952.png",
                "author": {
                    "self": "https://jira.com/rest/api/2/user?username=example.user",
                    "name": "example.user2",
                    "key": "example.user2",
                    "emailAddress": "example.user2@example.com",
                    "avatarUrls": {
                        "48x48": "https://jira.com/secure/useravatar?avatarId=12341",
                        "24x24": "https://jira.com/secure/useravatar?size=small&avatarId=12341",
                        "16x16": "https://jira.com/secure/useravatar?size=xsmall&avatarId=12341",
                        "32x32": "https://jira.com/secure/useravatar?size=medium&avatarId=12341"},
                        "displayName": "Example User2",
                        "active": true,
                        "timeZone": "GMT"
                    },
                "created": "2018-12-10T08:12:15.000+0000",
                "size": 117646,
                "mimeType": "image/png",
                "content": "https://jira.com/secure/attachment/45404/image-2018-12-10-09-12-14-952.png",
                "thumbnail": "https://jira.com/secure/thumbnail/45404/_thumb_45404.png"
            }],
            "created": "2018-07-11T16:56:12.000+0000",
            "creator": {
                "self": "https://jira.com/rest/api/2/user?accountId=123456%3A00000000-0000-0000-0000-000000000000",
                "name": "example.user",
                "key": "example.user",
                "displayName": "Example User",
                "active": true,
                "timeZone": "America/New_York",
                "emailAddress": "example.user@example.com",
                "avatarUrls": {
                    "48x48": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=48&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D48%26noRedirect%3Dtrue",
                    "24x24": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=24&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D24%26noRedirect%3Dtrue",
                    "16x16": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=16&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D16%26noRedirect%3Dtrue",
                    "32x32": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=32&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D32%26noRedirect%3Dtrue"
                }
            },
            "description": "Example Blocker Bug description.",
            "fixVersions": [{
                "self": "http://jira.com/rest/api/2/version/12345",
                "id": "12345",
                "description": "An excellent version",
                "name": "New Version 1",
                "archived": false,
                "released": true,
                "releaseDate": "2010-07-06",
                "overdue": true,
                "userReleaseDate": "6/Jul/2010",
                "projectId": 12345
            }],
            "issuelinks": [{
                "id": "12345",
                "self": "https://jira.com/rest/api/2/issueLink/12345",
                "type": {
                    "id": "10000",
                    "name": "Blocks",
                    "inward": "is blocked by",
                    "outward": "blocks",
                    "self": "https://jira.com/rest/api/2/issueLinkType/10000"
                },
                "inwardIssue": {
                    "id": "1234",
                    "key": "EXAMPLE-1",
                    "self": "https://jira.com/rest/api/2/issue/1234",
                    "fields": {
                        "summary": "An Example Issue",
                        "status": {
                            "self": "https://jira.com/rest/api/2/status/1",
                            "description": "the issue is open and ready for the assignee to start work on it.",
                            "iconurl": "https://jira.com/images/icons/statuses/open.png",
                            "name": "open",
                            "id": "1",
                            "statuscategory": {
                                "self": "https://jira.com/rest/api/2/statuscategory/2",
                                "id": 2,
                                "key": "new",
                                "colorName": "blue-gray",
                                "name": "To Do"
                            }
                        }
                    }
                }
            }],
            "priority": {
              "self": "https://jira.com/rest/api/2/priority/3",
              "iconUrl": "https://jira.com/images/icons/priorities/major.svg",
              "name": "P3",
              "id": "3"
            },
            "resolution": {
                "self": "https://jira.com/rest/api/2/resolution/3",
                "id": "3",
                "description": "The problem is a duplicate of an existing issue.",
                "name": "Duplicate"
            },
            "issuetype": {
                "self": "https://jira.com/rest/api/2/issuetype/1",
                "id": "1",
                "description": "A problem which impairs or prevents the functions of the product.",
                "iconUrl": "https://avatar-cdn.atlassian.com/secure/viewavatar?size=xsmall&avatarId=12345&avatarType=issuetype",
                "name": "Bug",
                "subtask": false,
                "avatarId": 12345
            },
            "labels": ["Label1", "Label2"],
            "priority": {
                "self": "https://jira.com/rest/api/2/priority/1",
                "iconUrl": "https://jira.com/images/icons/priorities/blocker.svg",
                "name": "Blocker",
                "id": "1"
            },
            "project": {
                "self": "https://jira.com/rest/api/2/project/12345",
                "id": "12345",
                "key": "EXAMPLE",
                "name": "Example",
                "avatarUrls": {
                    "48x48": "https://jira.com/secure/projectavatar?pid=12345&avatarId=12345",
                    "24x24": "https://jira.com/secure/projectavatar?size=small&pid=12345&avatarId=12345",
                    "16x16": "https://jira.com/secure/projectavatar?size=xsmall&pid=12345&avatarId=12345",
                    "32x32": "https://jira.com/secure/projectavatar?size=medium&pid=12345&avatarId=12345"
                }
            },
            "reporter": {
                "self": "https://jira.com/rest/api/2/user?accountId=123456%3A00000000-0000-0000-0000-000000000000",
                "name": "example.user",
                "key": "example.user",
                "displayName": "Example User",
                "active": true,
                "timeZone": "America/New_York",
                "emailAddress": "example.user@example.com",
                "avatarUrls": {
                    "48x48": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=48&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D48%26noRedirect%3Dtrue",
                    "24x24": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=24&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D24%26noRedirect%3Dtrue",
                    "16x16": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=16&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D16%26noRedirect%3Dtrue",
                    "32x32": "https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s=32&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D32%26noRedirect%3Dtrue"
                }
            },
            "status": {
                "self": "https://issues.jenkins-ci.org/rest/api/2/status/1",
                "description": "The issue is open and ready for the assignee to start work on it.",
                "iconUrl": "https://issues.jenkins-ci.org/images/icons/statuses/open.png",
                "name": "Open",
                "id": "1",
                "statusCategory": {
                    "self": "https://issues.jenkins-ci.org/rest/api/2/statuscategory/2",
                    "id": 2,
                    "key": "new",
                    "colorName": "blue-gray",
                    "name": "To Do"
                }
            },
            "summary": "Example Blocker Bug",
            "updated": "2018-07-11T16:56:12.000+0000",
            "resolutiondate": "2018-07-11T16:56:12.000+0000"
        }
    }"#;

    let credentials = Credentials::Basic("user".to_string(), "pwd".to_string());
    let jira = Jira::new(JIRA_HOST, credentials).unwrap();
    let issue: Issue = serde_json::from_str(issue_str).unwrap();

    let expected_date = Some("2018-07-11T16:56:12.000+0000".to_string());
    let expected_fix_versions = vec![
        Version {
            archived: false,
            description: "An excellent version".to_string(),
            id: "12345".to_string(),
            name: "New Version 1".to_string(),
            overdue: true,
            project_id: 12345,
            released: true,
            release_date: Some("2010-07-06".to_string()),
            self_link: "http://jira.com/rest/api/2/version/12345".to_string(),
            user_release_date: Some("6/Jul/2010".to_string()),
        }
    ];
    let expected_issue_type = Some(IssueType {
        description: "A problem which impairs or prevents the functions of the product.".to_string(),
        icon_url: "https://avatar-cdn.atlassian.com/secure/viewavatar?size=xsmall&avatarId=12345&avatarType=issuetype".to_string(),
        id: "1".to_string(),
        name: "Bug".to_string(),
        self_link: "https://jira.com/rest/api/2/issuetype/1".to_string(),
        subtask: false,
        avatar_id: 12345
    });
    let expected_permalink = format!("{}/browse/{}", JIRA_HOST, issue.key);
    let expected_priority = Some(Priority {
        icon_url: "https://jira.com/images/icons/priorities/blocker.svg".to_string(),
        id: "1".to_string(),
        name: "Blocker".to_string(),
        self_link: "https://jira.com/rest/api/2/priority/1".to_string(),
    });
    let expected_project = Some(Project {
        id: "12345".to_string(),
        key: "EXAMPLE".to_string(),
        name: "Example".to_string(),
        self_link: "https://jira.com/rest/api/2/project/12345".to_string(),
        avatar_urls: vec![
            ("48x48".to_string(), "https://jira.com/secure/projectavatar?pid=12345&avatarId=12345".to_string()),
            ("24x24".to_string(), "https://jira.com/secure/projectavatar?size=small&pid=12345&avatarId=12345".to_string()),
            ("16x16".to_string(), "https://jira.com/secure/projectavatar?size=xsmall&pid=12345&avatarId=12345".to_string()),
            ("32x32".to_string(), "https://jira.com/secure/projectavatar?size=medium&pid=12345&avatarId=12345".to_string()),
        ].into_iter().collect()
    });
    let expected_status = Some(Status {
        description: "The issue is open and ready for the assignee to start work on it.".to_string(),
        icon_url: "https://issues.jenkins-ci.org/images/icons/statuses/open.png".to_string(),
        id: "1".to_string(),
        name: "Open".to_string(),
        self_link: "https://issues.jenkins-ci.org/rest/api/2/status/1".to_string(),
    });
    let expected_user = Some(User {
        active: true,
        avatar_urls: ["16", "24", "32", "48"]
            .iter()
            .map(|&s| (format!("{}x{}", s, s), format!("https://avatar-cdn.atlassian.com/00000000000000000000000000000000?s={}&d=https%3A%2F%2Fsecure.gravatar.com%2Favatar%2F00000000000000000000000000000000%3Fd%3Dmm%26s%3D{}%26noRedirect%3Dtrue", s, s)))
            .collect(),
        display_name: "Example User".to_string(),
        email_address: "example.user@example.com".to_string(),
        key: Some("example.user".to_string()),
        name: "example.user".to_string(),
        self_link: "https://jira.com/rest/api/2/user?accountId=123456%3A00000000-0000-0000-0000-000000000000".to_string(),
        timezone: Some("America/New_York".to_string()),
    });

    assert_eq!(issue.assignee(), expected_user);
    assert_eq!(issue.created(), expected_date);
    assert_eq!(issue.creator(), expected_user);
    assert_eq!(issue.description(), Some("Example Blocker Bug description.".to_string()));
    assert_eq!(issue.fix_versions(), expected_fix_versions);
    assert_eq!(issue.issue_type(), expected_issue_type);
    assert_eq!(issue.labels(), vec!["Label1", "Label2"]);
    assert_eq!(issue.permalink(&jira), expected_permalink);
    assert_eq!(issue.priority(), expected_priority);
    assert_eq!(issue.project(), expected_project);
    assert_eq!(issue.reporter(), expected_user);
    assert_eq!(issue.resolution_date(), expected_date);
    assert_eq!(issue.status(), expected_status);
    assert_eq!(issue.summary(), Some("Example Blocker Bug".to_string()));
    assert_eq!(issue.updated(), expected_date);
    // TODO links, resolution, attachment, comment
}
